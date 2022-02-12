use std::sync::Arc;

fn main() {
    let buffer = Arc::new(Buffer { });
    let another_buffer = Arc::new(AnotherBuffer { });
 
    takes_collection(buffer.clone());
    takes_collection(vec![buffer.clone(), buffer.clone()]);
    takes_collection((buffer.clone(), buffer.clone()));

    let buffer_trait_obj = buffer.clone() as Arc<dyn BufferAccess>;
    let another_buffer_trait_obj = another_buffer.clone() as Arc<dyn BufferAccess>;

    takes_collection(buffer_trait_obj.clone());
    takes_collection(vec![buffer_trait_obj.clone(), another_buffer.clone()]);
    takes_collection((buffer_trait_obj.clone(), another_buffer_trait_obj.clone()));
    takes_collection((buffer_trait_obj.clone(), another_buffer.clone()));
}

pub struct Buffer { }
pub struct AnotherBuffer { }

pub trait BufferAccess { }
impl BufferAccess for Buffer { }
impl BufferAccess for AnotherBuffer { }

pub trait BufferAccessObject {
    fn as_buffer_access_object(&self) -> Arc<dyn BufferAccess>;
}

impl BufferAccessObject for Arc<dyn BufferAccess> {
    fn as_buffer_access_object(&self) -> Arc<dyn BufferAccess> {
        self.clone()
    }
}

impl BufferAccessObject for Arc<Buffer> {
    fn as_buffer_access_object(&self) -> Arc<dyn BufferAccess> {
        self.clone()
    }
}

impl BufferAccessObject for Arc<AnotherBuffer> {
    fn as_buffer_access_object(&self) -> Arc<dyn BufferAccess> {
        self.clone()
    }
}

pub trait VertexBuffersCollection {
    fn into_vec(self) -> Vec<Arc<dyn BufferAccess>>;
}

impl VertexBuffersCollection for () {
    fn into_vec(self) -> Vec<Arc<dyn BufferAccess>> {
        Vec::new()
    }
}

impl<T: BufferAccessObject> VertexBuffersCollection for T {
    fn into_vec(self) -> Vec<Arc<dyn BufferAccess>> {
        vec![self.as_buffer_access_object()]
    }
}

impl<T: BufferAccessObject> VertexBuffersCollection for Vec<T> {
    fn into_vec(self) -> Vec<Arc<dyn BufferAccess>> {
        self.into_iter().map(|src| src.as_buffer_access_object()).collect()
    }
}

macro_rules! impl_collection {
    ($first:ident $(, $others:ident)+) => (
        impl<$first$(, $others)+> VertexBuffersCollection for ($first, $($others),+)
            where $first: BufferAccessObject
                  $(, $others: BufferAccessObject)*
        {
            #[inline]
            fn into_vec(self) -> Vec<Arc<dyn BufferAccess>> {
                #![allow(non_snake_case)]

                let ($first, $($others,)*) = self;
                let mut list = Vec::new();
                list.push($first.as_buffer_access_object());

                $(
                    list.push($others.as_buffer_access_object());
                )+

                list
            }
        }

        impl_collection!($($others),+);
    );

    ($i:ident) => ();
}

impl_collection!(Z, Y, X, W, V, U, T, S, R, Q, P, O, N, M, L, K, J, I, H, G, F, E, D, C, B, A);

fn takes_collection<C: VertexBuffersCollection>(collection: C) {
    let _vec: Vec<Arc<dyn BufferAccess>> = collection.into_vec();
}
