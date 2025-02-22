use std::{collections::HashMap, sync::atomic::AtomicU64};

use anno_lua::Anno;
use mlua::{FromLua, UserData};

use crate::Register;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Duration {
    dur: std::time::Duration,
}

impl FromLua for Duration {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        let mlua::Value::UserData(ud) = value else {
            return Err(mlua::Error::runtime(format!(
                "expected Duration, got: {}",
                value.type_name(),
            )));
        };
        ud.borrow::<Self>().map(|c| *c)
    }
}

impl UserData for Duration {
    fn add_methods<M>(methods: &mut M)
    where
        M: mlua::UserDataMethods<Self>,
    {
        methods.add_function("from_secs", |_lua, secs: u64| {
            Ok(Self {
                dur: std::time::Duration::from_secs(secs),
            })
        });

        methods.add_function("from_millis", |_lua, millis: u64| {
            Ok(Self {
                dur: std::time::Duration::from_millis(millis),
            })
        });

        methods.add_function("from_micros", |_lua, micros: u64| {
            Ok(Self {
                dur: std::time::Duration::from_micros(micros),
            })
        });
    }
}

impl Anno for Duration {
    fn lua_type() -> anno_lua::Type {
        anno_lua::Type::Class(anno_lua::Class {
            exact: true,
            docs: &["Duration wrapper"],
            name: "Duration",
            fields: &[
                anno_lua::Field {
                    name: "from_secs",
                    ty: "fun(secs: integer): Duration",
                    docs: &["Creates a new duration from seconds"],
                },
                anno_lua::Field {
                    name: "from_millis",
                    ty: "fun(millis: integer): Duration",
                    docs: &["Creates a new duration from milliseconds"],
                },
                anno_lua::Field {
                    name: "from_micros",
                    ty: "fun(micros: integer): Duration",
                    docs: &["Creates a new duration from microseconds"],
                },
            ],
        })
    }
}

impl Register for Duration {
    const NAME: &'static str = "Duration";
}

impl From<Duration> for std::time::Duration {
    fn from(value: Duration) -> Self {
        value.dur
    }
}

pub struct Runtime;

impl Register for Runtime {
    const NAME: &'static str = "Runtime";
}

impl Anno for Runtime {
    fn lua_type() -> anno_lua::Type {
        anno_lua::Type::Class(anno_lua::Class {
            exact: true,
            docs: &["An async runtime"],
            name: "Runtime",
            fields: &[
                anno_lua::Field {
                    name: "sleep",
                    ty: "fun(dur: Duration): nil",
                    docs: &["sleeps for a specific duration"],
                },
                anno_lua::Field {
                    name: "spawn",
                    ty: "fun(task: (fun(): nil) | thread): integer",
                    docs: &[
                        "spawns a function or coroutine",
                        "",
                        "this returns an id that you can use to stop the task",
                    ],
                },
                anno_lua::Field {
                    name: "stop",
                    ty: "fun(integer): boolean",
                    docs: &["attempts to stop a running task"],
                },
            ],
        })
    }
}

impl mlua::UserData for Runtime {
    fn add_methods<M>(methods: &mut M)
    where
        M: mlua::UserDataMethods<Self>,
    {
        methods.add_async_function("sleep", |_lua, duration: Duration| async move {
            tokio::time::sleep(duration.into()).await;
            Ok(())
        });

        methods.add_function("spawn", |lua, value: mlua::Value| {
            use tokio_stream::StreamExt as _;

            let handle = match value {
                mlua::Value::Function(function) => {
                    let future = function.call_async::<()>(());
                    tokio::spawn(async move {
                        let _ = future.await;
                    })
                }

                mlua::Value::Thread(thread) => {
                    let mut stream = thread.into_async::<()>(());
                    tokio::spawn(async move {
                        while let Some(..) = stream.next().await {
                            //
                        }
                    })
                }

                _ => {
                    return Err(mlua::Error::runtime(
                        "only functions or coroutines can be spawned",
                    ));
                }
            };

            let id = lua
                .app_data_mut::<RunningTasks>()
                .unwrap()
                .push(handle.abort_handle());
            Ok(id)
        });

        methods.add_function("stop", |lua, id: Option<u64>| {
            let Some(id) = id else { return Ok(false) };
            let mut tasks = lua.app_data_mut::<RunningTasks>().unwrap();
            Ok(tasks.shutdown(id))
        });
    }
}

#[derive(Default)]
pub struct RunningTasks {
    tasks: HashMap<u64, tokio::task::AbortHandle>,
}

impl RunningTasks {
    fn push(&mut self, handle: tokio::task::AbortHandle) -> u64 {
        static TASK_ID: AtomicU64 = AtomicU64::new(0);
        let id = TASK_ID.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        self.tasks.insert(id, handle);
        id
    }

    fn shutdown(&mut self, task: u64) -> bool {
        if let Some(task) = self.tasks.remove(&task) {
            task.abort();
            return true;
        }
        false
    }

    pub fn stop_all(&mut self) {
        for (_, task) in self.tasks.drain() {
            task.abort();
        }
    }
}
