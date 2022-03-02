use std::array::TryFromSliceError;

use anyhow::Result;
use hex::ToHex;
use macro_rules_attribute::macro_rules_attribute;
use paste::paste;
use serde::Serialize;

type Fb = [u8; 3];

macro_rules! test {
    (
        $(#[$struct_meta:meta])*
        $struct_vis:vis
        struct $StructName:ident {
            $(
                $(#[$field_meta:meta])*
                $field_vis:vis $field_name:ident : $field_ty:ty
            ),* $(,)?
        }
    ) => (
        // generate the struct definition we have been given
        $(#[$struct_meta])*
        $struct_vis
        struct $StructName {
            $(
                $(#[$field_meta])*
                $field_vis $field_name: $field_ty,
            )*
        }

        paste! {
            $(#[$struct_meta])*
            $struct_vis
            struct [< $StructName Db >] {
                $(
                    $(#[$field_meta])*
                    $field_vis $field_name: ty!($field_ty),
                )*
            }

            // moves the Vec<u8> in fixed size array using try_into from Slice in std::array
            impl TryFrom<[< $StructName Db >]> for $StructName {
                type Error = TryFromSliceError;
                fn try_from(st: [< $StructName Db >]) -> Result<Self, Self::Error> {
                    Ok(Self {
                        $(
                            // $field_name: st.$field_name,
                            $field_name: sel!(st, $field_name, $field_ty),
                        )*
                    })
                }
            }

        }
    )
}
// moves the Vec<u8> in fixed size array
// impl TryFrom<DbTransDb> for DbTrans {
//     type Error = TryFromSliceError;
//     fn try_from(st: DbTransDb) -> Result<Self, Self::Error> {
//         Ok(Self {
//             id: st.id,
//             hash: if true {
//                 st.hash.as_slice().try_into()?
//             } else {
//             },
//         })
//     }
// }

macro_rules! sel {
    ($st:ident, $field_name:ident, Fb) => {
        $st.$field_name.as_slice().try_into()?
    };
    ($st:ident, $field_name:ident, $field_ty:ty) => {
        $st.$field_name
    };
}
macro_rules! ty {
    (Fb) => {
        Vec<u8>
    };
    ($field_ty:ty) => {
        $field_ty
    };
}
#[macro_rules_attribute(test!)]
#[derive(Debug, PartialEq, Serialize)]
pub struct DbTrans {
    pub id: i64,
    #[serde(rename = "identifier", serialize_with = "buffer_to_hex")]
    pub hash: Fb,
}
pub fn buffer_to_hex<S>(buffer: &[u8], serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let hex_str = buffer.encode_hex::<String>();
    serializer.serialize_str(&hex_str)
}
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}
fn main() -> Result<()> {
    // let mut w = Vec::new();
    // write!(&mut w, "test").unwrap();
    // write!(&mut w, "formatted {}", "arguments").unwrap();
    // let a: ty!() = 1;
    let _ = stringify!(u8);

    let db = DbTransDb {
        id: 1,
        hash: vec![1, 2, 3],
    };
    let db: DbTrans = db.try_into()?;
    print_type_of(&db.hash);
    println!("{}", serde_json::to_string_pretty(&db).unwrap());

    println!("{:?} {}", "asd", "asd");

    let x: Option<Vec<u8>> = Some(vec![1, 2, 3]);
    let y = if x.is_some() {
        Some(<[u8; 3]>::try_from(x.unwrap()).unwrap())
    } else {
        None
    };

    Ok(())
}
