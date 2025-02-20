use std::{collections::HashMap, sync::atomic::AtomicU64, time::Duration};

pub struct Runtime;

// Method {
//     kind: Kind::Async,
//     name: "sleep_ms",
//     args: "millis: integer",
//     params: "millis",
//     returns: "nil",
//     doc: "sleep for `millis`",
// },
// Method {
//     kind: Kind::Sync,
//     name: "spawn",
//     args: "task: fun():nil | thread",
//     params: "task",
//     returns: "integer",
//     doc: "spawns an async tasks, returning its id",
// },
// Method {
//     kind: Kind::Sync,
//     name: "stop",
//     args: "id: integer?",
//     params: "id",
//     returns: "boolean",
//     doc: "attempts to stop a running task",
// },

impl mlua::UserData for Runtime {
    fn add_methods<M>(methods: &mut M)
    where
        M: mlua::UserDataMethods<Self>,
    {
        methods.add_async_function("sleep_ms", |_lua, ms: u32| async move {
            tokio::time::sleep(Duration::from_millis(ms as u64)).await;
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
                    ))
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
