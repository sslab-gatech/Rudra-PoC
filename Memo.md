# Memo

## UnsafeDestructor

## SendSyncChecker

### When things are valid

- Send for Container<T>
    * Incorrect in most of the case
    * `T: Send` is required for `fn(any Container) -> T / &mut T`
- Sync for Container<T>
    * `T: Sync` is required for `fn(&Container) -> &T` (usually `Deref`)
    * `T: Send` is required for `fn(&Container) -> T` (atomic replacement)

- Mutex<T>
    * `&Self -> &mut T`
    * T:Send for Send
    * T:Send for Sync
- RwLock<T>
    * T:Send for Send
    * T:Send+Sync for Sync
- Box<T>/Vec<T>/[T]
    * T:Send for Send
    * (T:Sync for Sync)
- &T
    * T:Sync for Send
- &mut T
    * T:Send for Send
- Cow<T>
    * T:Sync(borrowed)/Send(owned) for Send
    * T:Sync/Sync for Sync
- Arc<T>
    * T:Send+Sync for Send
    * T:Send+Sync for Sync
    * T:Send bound is needed because `&Arc` can be sent and cloned in another thread
