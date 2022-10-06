/// i64 の id と GraphQL の id を相互に変換するボイラープレートを実装するマクロ
#[macro_export]
macro_rules! impl_i64_convert {
    ($id:tt) => {
        impl TryInto<$id> for i64 {
            type Error = std::num::TryFromIntError;

            fn try_into(self) -> std::result::Result<$id, Self::Error> {
                Ok($id(self.try_into()?))
            }
        }

        impl From<$id> for i64 {
            fn from(v: $id) -> Self {
                Self::from(v.0)
            }
        }
    };
}
