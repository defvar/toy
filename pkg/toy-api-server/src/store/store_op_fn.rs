// use crate::store::error::StoreError;
// use crate::store::{
//     Find, FindOption, List, ListOption, StoreConnection, StoreOps, StoreOpsFactory,
// };
// use core::marker::PhantomData;
// use std::fmt::Debug;
// use std::future::Future;
// use toy_core::data::Value;
//
// pub fn fn_find<C, F, Err, Fut>(f: F) -> FnFind<C, F, Err, Fut>
// where
//     C: StoreConnection,
//     F: Fn(C, String, FindOption) -> Fut,
//     Err: Debug,
//     Fut: Future<Output = Result<Option<Value>, Err>> + Send,
// {
//     FnFind { f, _t: PhantomData }
// }
//
// pub fn fn_list<C, F, Err, Fut>(f: F) -> FnList<C, F, Err, Fut>
// where
//     C: StoreConnection,
//     F: Fn(C, String, ListOption) -> Fut,
//     Err: Debug,
//     Fut: Future<Output = Result<Vec<Value>, Err>> + Send,
// {
//     FnList { f, _t: PhantomData }
// }
//
// pub fn fn_store_ops<C, TFind, TList>(find: TFind, list: TList) -> FnStoreOps<C, TFind, TList>
// where
//     C: StoreConnection,
//     TFind: Find + Send + Sync,
//     TList: List + Send + Sync,
// {
//     FnStoreOps {
//         find,
//         list,
//         _t: PhantomData,
//     }
// }
//
// pub fn fn_ops_factory<C, S, F, Connect>(
//     f: F,
//     connect: Connect,
// ) -> FnStoreOpsFactory<C, S, F, Connect>
// where
//     C: StoreConnection,
//     F: Fn() -> Result<S, StoreError> + Send + Sync,
//     Connect: Fn() -> Result<C, StoreError> + Send + Sync,
// {
//     FnStoreOpsFactory {
//         f,
//         connect,
//         _t: PhantomData,
//     }
// }
//
// #[derive(Clone)]
// pub struct FnFind<C, F, Err, Fut> {
//     f: F,
//     _t: PhantomData<(C, Err, Fut)>,
// }
//
// impl<C, F, Err, Fut> Find for FnFind<C, F, Err, Fut>
// where
//     C: StoreConnection,
//     F: Fn(C, String, FindOption) -> Fut,
//     Err: Debug,
//     Fut: Future<Output = Result<Option<Value>, Err>> + Send,
// {
//     type Con = C;
//     type T = Fut;
//     type Err = Err;
//
//     fn find(&self, con: Self::Con, key: String, opt: FindOption) -> Self::T {
//         (self.f)(con, key, opt)
//     }
// }
//
// #[derive(Clone)]
// pub struct FnList<C, F, Err, Fut> {
//     f: F,
//     _t: PhantomData<(C, Err, Fut)>,
// }
//
// impl<C, F, Err, Fut> List for FnList<C, F, Err, Fut>
// where
//     C: StoreConnection,
//     F: Fn(C, String, ListOption) -> Fut,
//     Err: Debug,
//     Fut: Future<Output = Result<Vec<Value>, Err>> + Send,
// {
//     type Con = C;
//     type T = Fut;
//     type Err = Err;
//
//     fn list(&self, con: Self::Con, prefix: String, opt: ListOption) -> Self::T {
//         (self.f)(con, prefix, opt)
//     }
// }
//
// pub struct FnStoreOps<C, TFind, TList> {
//     find: TFind,
//     list: TList,
//     _t: PhantomData<C>,
// }
//
// impl<C, TFind, TList> StoreOps<C> for FnStoreOps<C, TFind, TList>
// where
//     C: StoreConnection,
//     TFind: Find<Con = C> + Send + Sync,
//     TList: List<Con = C> + Send + Sync,
// {
// }
//
// impl<C, TFind, TList> Find for FnStoreOps<C, TFind, TList>
// where
//     C: StoreConnection,
//     TFind: Find<Con = C> + Send + Sync,
//     TList: List<Con = C> + Send + Sync,
// {
//     type Con = TFind::Con;
//     type T = TFind::T;
//     type Err = TFind::Err;
//
//     fn find(&self, con: Self::Con, key: String, opt: FindOption) -> Self::T {
//         self.find.find(con, key, opt)
//     }
// }
//
// impl<C, TFind, TList> List for FnStoreOps<C, TFind, TList>
// where
//     C: StoreConnection,
//     TFind: Find<Con = C> + Send + Sync,
//     TList: List<Con = C> + Send + Sync,
// {
//     type Con = TList::Con;
//     type T = TList::T;
//     type Err = TList::Err;
//
//     fn list(&self, con: Self::Con, prefix: String, opt: ListOption) -> Self::T {
//         self.list.list(con, prefix, opt)
//     }
// }
//
// pub struct FnStoreOpsFactory<C, S, F, Connect> {
//     f: F,
//     connect: Connect,
//     _t: PhantomData<(C, S)>,
// }
//
// impl<C, S, F, Connect> StoreOpsFactory<C> for FnStoreOpsFactory<C, S, F, Connect>
// where
//     C: StoreConnection,
//     S: StoreOps<C>,
//     F: Fn() -> Result<S, StoreError> + Send + Sync,
//     Connect: Fn() -> Result<C, StoreError> + Send + Sync,
// {
//     type Ops = S;
//
//     fn create(&self) -> Result<Self::Ops, StoreError> {
//         (self.f)()
//     }
//
//     fn connect(&self) -> Result<C, StoreError> {
//         (self.connect)()
//     }
// }
//
// impl<C, S, F: Clone, Connect: Clone> Clone for FnStoreOpsFactory<C, S, F, Connect> {
//     fn clone(&self) -> Self {
//         FnStoreOpsFactory {
//             f: self.f.clone(),
//             connect: self.connect.clone(),
//             _t: PhantomData,
//         }
//     }
// }
