# Memo

## UnsafeDestructor

## SendSyncChecker

### When things are valid

- Send for Container<T>
    * Incorrect in most of the case
    * `T: Send` is required for `fn(any Container) -> T`
- Sync for Container<T>
    * `T: Sync` is required for `fn(&Container) -> &T` (usually `Deref`)
    * `T: Send` is required for `fn(&Container) -> T` (atomic replacement)

- Mutex
    * `&Self -> &mut T`
    * T:Send for Send
    * T:Send for Sync
- RwLock
    * T:Send for Send
    * T:Send+Sync for Sync
- Box/Vec/slice
    * T:Send for Send
    * (T:Sync for Sync)
- &
    * T:Sync for Send
- &mut
    * T:Send for Send
- Cow
    * T:Sync(borrowed)/Send(owned) for Send
    * T:Sync/Sync for Sync
- Arc
    * T:Send+Sync for Send
    * T:Send+Sync for Sync
    * T:Send bound is needed because `&Arc` can be sent and cloned in another thread
