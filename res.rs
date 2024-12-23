pub mod product_entity {
    use rbatis::crud;
    use rs_service_util::{time::get_current_time_fmt, Status};
    use serde::{Deserialize, Serialize};
    use utility_types::{Omit, Pick};
    use utoipa::ToSchema;
    #[omit(
        arg(
            ident = ProductItem,
            fields(status, price),
            derive(Clone, Debug, Serialize, Deserialize),
            forward_attrs(),
        ),
        arg(
            ident = CreateProductReqData,
            fields(status, id, create_time, update_time),
            derive(Clone, Debug, Serialize, Deserialize, ToSchema),
            forward_attrs()
        ),
    )]
    pub struct ProductEntity {
        pub id: Option<i32>,
        pub create_time: String,
        pub update_time: String,
        pub name: String,
        pub status: u8,
        pub price: f64,
        pub picture: String,
        pub count: i32,
        pub ext: String,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ProductEntity {
        #[inline]
        fn clone(&self) -> ProductEntity {
            ProductEntity {
                id: ::core::clone::Clone::clone(&self.id),
                create_time: ::core::clone::Clone::clone(&self.create_time),
                update_time: ::core::clone::Clone::clone(&self.update_time),
                name: ::core::clone::Clone::clone(&self.name),
                status: ::core::clone::Clone::clone(&self.status),
                price: ::core::clone::Clone::clone(&self.price),
                picture: ::core::clone::Clone::clone(&self.picture),
                count: ::core::clone::Clone::clone(&self.count),
                ext: ::core::clone::Clone::clone(&self.ext),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ProductEntity {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "id",
                "create_time",
                "update_time",
                "name",
                "status",
                "price",
                "picture",
                "count",
                "ext",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &self.id,
                &self.create_time,
                &self.update_time,
                &self.name,
                &self.status,
                &self.price,
                &self.picture,
                &self.count,
                &&self.ext,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(
                f,
                "ProductEntity",
                names,
                values,
            )
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for ProductEntity {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "ProductEntity",
                    false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "create_time",
                    &self.create_time,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "update_time",
                    &self.update_time,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "name",
                    &self.name,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "status",
                    &self.status,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "price",
                    &self.price,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "picture",
                    &self.picture,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "count",
                    &self.count,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "ext",
                    &self.ext,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for ProductEntity {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __field6,
                    __field7,
                    __field8,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            4u64 => _serde::__private::Ok(__Field::__field4),
                            5u64 => _serde::__private::Ok(__Field::__field5),
                            6u64 => _serde::__private::Ok(__Field::__field6),
                            7u64 => _serde::__private::Ok(__Field::__field7),
                            8u64 => _serde::__private::Ok(__Field::__field8),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "id" => _serde::__private::Ok(__Field::__field0),
                            "create_time" => _serde::__private::Ok(__Field::__field1),
                            "update_time" => _serde::__private::Ok(__Field::__field2),
                            "name" => _serde::__private::Ok(__Field::__field3),
                            "status" => _serde::__private::Ok(__Field::__field4),
                            "price" => _serde::__private::Ok(__Field::__field5),
                            "picture" => _serde::__private::Ok(__Field::__field6),
                            "count" => _serde::__private::Ok(__Field::__field7),
                            "ext" => _serde::__private::Ok(__Field::__field8),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"id" => _serde::__private::Ok(__Field::__field0),
                            b"create_time" => _serde::__private::Ok(__Field::__field1),
                            b"update_time" => _serde::__private::Ok(__Field::__field2),
                            b"name" => _serde::__private::Ok(__Field::__field3),
                            b"status" => _serde::__private::Ok(__Field::__field4),
                            b"price" => _serde::__private::Ok(__Field::__field5),
                            b"picture" => _serde::__private::Ok(__Field::__field6),
                            b"count" => _serde::__private::Ok(__Field::__field7),
                            b"ext" => _serde::__private::Ok(__Field::__field8),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<ProductEntity>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = ProductEntity;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct ProductEntity",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            Option<i32>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct ProductEntity with 9 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct ProductEntity with 9 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct ProductEntity with 9 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct ProductEntity with 9 elements",
                                    ),
                                );
                            }
                        };
                        let __field4 = match _serde::de::SeqAccess::next_element::<
                            u8,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        4usize,
                                        &"struct ProductEntity with 9 elements",
                                    ),
                                );
                            }
                        };
                        let __field5 = match _serde::de::SeqAccess::next_element::<
                            f64,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        5usize,
                                        &"struct ProductEntity with 9 elements",
                                    ),
                                );
                            }
                        };
                        let __field6 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        6usize,
                                        &"struct ProductEntity with 9 elements",
                                    ),
                                );
                            }
                        };
                        let __field7 = match _serde::de::SeqAccess::next_element::<
                            i32,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        7usize,
                                        &"struct ProductEntity with 9 elements",
                                    ),
                                );
                            }
                        };
                        let __field8 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        8usize,
                                        &"struct ProductEntity with 9 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(ProductEntity {
                            id: __field0,
                            create_time: __field1,
                            update_time: __field2,
                            name: __field3,
                            status: __field4,
                            price: __field5,
                            picture: __field6,
                            count: __field7,
                            ext: __field8,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<Option<i32>> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field3: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field4: _serde::__private::Option<u8> = _serde::__private::None;
                        let mut __field5: _serde::__private::Option<f64> = _serde::__private::None;
                        let mut __field6: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field7: _serde::__private::Option<i32> = _serde::__private::None;
                        let mut __field8: _serde::__private::Option<String> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<i32>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "create_time",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "update_time",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::__private::Option::is_some(&__field4) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("status"),
                                        );
                                    }
                                    __field4 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<u8>(&mut __map)?,
                                    );
                                }
                                __Field::__field5 => {
                                    if _serde::__private::Option::is_some(&__field5) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("price"),
                                        );
                                    }
                                    __field5 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<f64>(&mut __map)?,
                                    );
                                }
                                __Field::__field6 => {
                                    if _serde::__private::Option::is_some(&__field6) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "picture",
                                            ),
                                        );
                                    }
                                    __field6 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                __Field::__field7 => {
                                    if _serde::__private::Option::is_some(&__field7) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("count"),
                                        );
                                    }
                                    __field7 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<i32>(&mut __map)?,
                                    );
                                }
                                __Field::__field8 => {
                                    if _serde::__private::Option::is_some(&__field8) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("ext"),
                                        );
                                    }
                                    __field8 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("id")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("create_time")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("update_time")?
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("name")?
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::__private::Some(__field4) => __field4,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("status")?
                            }
                        };
                        let __field5 = match __field5 {
                            _serde::__private::Some(__field5) => __field5,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("price")?
                            }
                        };
                        let __field6 = match __field6 {
                            _serde::__private::Some(__field6) => __field6,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("picture")?
                            }
                        };
                        let __field7 = match __field7 {
                            _serde::__private::Some(__field7) => __field7,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("count")?
                            }
                        };
                        let __field8 = match __field8 {
                            _serde::__private::Some(__field8) => __field8,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("ext")?
                            }
                        };
                        _serde::__private::Ok(ProductEntity {
                            id: __field0,
                            create_time: __field1,
                            update_time: __field2,
                            name: __field3,
                            status: __field4,
                            price: __field5,
                            picture: __field6,
                            count: __field7,
                            ext: __field8,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "id",
                    "create_time",
                    "update_time",
                    "name",
                    "status",
                    "price",
                    "picture",
                    "count",
                    "ext",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "ProductEntity",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<ProductEntity>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    pub struct ProductItem {
        pub id: Option<i32>,
        pub create_time: String,
        pub update_time: String,
        pub name: String,
        pub picture: String,
        pub count: i32,
        pub ext: String,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for ProductItem {
        #[inline]
        fn clone(&self) -> ProductItem {
            ProductItem {
                id: ::core::clone::Clone::clone(&self.id),
                create_time: ::core::clone::Clone::clone(&self.create_time),
                update_time: ::core::clone::Clone::clone(&self.update_time),
                name: ::core::clone::Clone::clone(&self.name),
                picture: ::core::clone::Clone::clone(&self.picture),
                count: ::core::clone::Clone::clone(&self.count),
                ext: ::core::clone::Clone::clone(&self.ext),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for ProductItem {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "id",
                "create_time",
                "update_time",
                "name",
                "picture",
                "count",
                "ext",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &self.id,
                &self.create_time,
                &self.update_time,
                &self.name,
                &self.picture,
                &self.count,
                &&self.ext,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(
                f,
                "ProductItem",
                names,
                values,
            )
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for ProductItem {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "ProductItem",
                    false as usize + 1 + 1 + 1 + 1 + 1 + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "id",
                    &self.id,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "create_time",
                    &self.create_time,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "update_time",
                    &self.update_time,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "name",
                    &self.name,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "picture",
                    &self.picture,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "count",
                    &self.count,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "ext",
                    &self.ext,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for ProductItem {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __field6,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            4u64 => _serde::__private::Ok(__Field::__field4),
                            5u64 => _serde::__private::Ok(__Field::__field5),
                            6u64 => _serde::__private::Ok(__Field::__field6),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "id" => _serde::__private::Ok(__Field::__field0),
                            "create_time" => _serde::__private::Ok(__Field::__field1),
                            "update_time" => _serde::__private::Ok(__Field::__field2),
                            "name" => _serde::__private::Ok(__Field::__field3),
                            "picture" => _serde::__private::Ok(__Field::__field4),
                            "count" => _serde::__private::Ok(__Field::__field5),
                            "ext" => _serde::__private::Ok(__Field::__field6),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"id" => _serde::__private::Ok(__Field::__field0),
                            b"create_time" => _serde::__private::Ok(__Field::__field1),
                            b"update_time" => _serde::__private::Ok(__Field::__field2),
                            b"name" => _serde::__private::Ok(__Field::__field3),
                            b"picture" => _serde::__private::Ok(__Field::__field4),
                            b"count" => _serde::__private::Ok(__Field::__field5),
                            b"ext" => _serde::__private::Ok(__Field::__field6),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<ProductItem>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = ProductItem;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct ProductItem",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            Option<i32>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct ProductItem with 7 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct ProductItem with 7 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct ProductItem with 7 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct ProductItem with 7 elements",
                                    ),
                                );
                            }
                        };
                        let __field4 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        4usize,
                                        &"struct ProductItem with 7 elements",
                                    ),
                                );
                            }
                        };
                        let __field5 = match _serde::de::SeqAccess::next_element::<
                            i32,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        5usize,
                                        &"struct ProductItem with 7 elements",
                                    ),
                                );
                            }
                        };
                        let __field6 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        6usize,
                                        &"struct ProductItem with 7 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(ProductItem {
                            id: __field0,
                            create_time: __field1,
                            update_time: __field2,
                            name: __field3,
                            picture: __field4,
                            count: __field5,
                            ext: __field6,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<Option<i32>> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field3: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field4: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field5: _serde::__private::Option<i32> = _serde::__private::None;
                        let mut __field6: _serde::__private::Option<String> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("id"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<i32>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "create_time",
                                            ),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "update_time",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::__private::Option::is_some(&__field4) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "picture",
                                            ),
                                        );
                                    }
                                    __field4 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                __Field::__field5 => {
                                    if _serde::__private::Option::is_some(&__field5) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("count"),
                                        );
                                    }
                                    __field5 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<i32>(&mut __map)?,
                                    );
                                }
                                __Field::__field6 => {
                                    if _serde::__private::Option::is_some(&__field6) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("ext"),
                                        );
                                    }
                                    __field6 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("id")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("create_time")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("update_time")?
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("name")?
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::__private::Some(__field4) => __field4,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("picture")?
                            }
                        };
                        let __field5 = match __field5 {
                            _serde::__private::Some(__field5) => __field5,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("count")?
                            }
                        };
                        let __field6 = match __field6 {
                            _serde::__private::Some(__field6) => __field6,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("ext")?
                            }
                        };
                        _serde::__private::Ok(ProductItem {
                            id: __field0,
                            create_time: __field1,
                            update_time: __field2,
                            name: __field3,
                            picture: __field4,
                            count: __field5,
                            ext: __field6,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "id",
                    "create_time",
                    "update_time",
                    "name",
                    "picture",
                    "count",
                    "ext",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "ProductItem",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<ProductItem>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl From<ProductEntity> for ProductItem {
        fn from(src: ProductEntity) -> Self {
            Self {
                id: src.id,
                create_time: src.create_time,
                update_time: src.update_time,
                name: src.name,
                picture: src.picture,
                count: src.count,
                ext: src.ext,
            }
        }
    }
    pub struct CreateProductReqData {
        pub name: String,
        pub price: f64,
        pub picture: String,
        pub count: i32,
        pub ext: String,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for CreateProductReqData {
        #[inline]
        fn clone(&self) -> CreateProductReqData {
            CreateProductReqData {
                name: ::core::clone::Clone::clone(&self.name),
                price: ::core::clone::Clone::clone(&self.price),
                picture: ::core::clone::Clone::clone(&self.picture),
                count: ::core::clone::Clone::clone(&self.count),
                ext: ::core::clone::Clone::clone(&self.ext),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for CreateProductReqData {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field5_finish(
                f,
                "CreateProductReqData",
                "name",
                &self.name,
                "price",
                &self.price,
                "picture",
                &self.picture,
                "count",
                &self.count,
                "ext",
                &&self.ext,
            )
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for CreateProductReqData {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "CreateProductReqData",
                    false as usize + 1 + 1 + 1 + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "name",
                    &self.name,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "price",
                    &self.price,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "picture",
                    &self.picture,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "count",
                    &self.count,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "ext",
                    &self.ext,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for CreateProductReqData {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            4u64 => _serde::__private::Ok(__Field::__field4),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "name" => _serde::__private::Ok(__Field::__field0),
                            "price" => _serde::__private::Ok(__Field::__field1),
                            "picture" => _serde::__private::Ok(__Field::__field2),
                            "count" => _serde::__private::Ok(__Field::__field3),
                            "ext" => _serde::__private::Ok(__Field::__field4),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"name" => _serde::__private::Ok(__Field::__field0),
                            b"price" => _serde::__private::Ok(__Field::__field1),
                            b"picture" => _serde::__private::Ok(__Field::__field2),
                            b"count" => _serde::__private::Ok(__Field::__field3),
                            b"ext" => _serde::__private::Ok(__Field::__field4),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<CreateProductReqData>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = CreateProductReqData;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct CreateProductReqData",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct CreateProductReqData with 5 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            f64,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct CreateProductReqData with 5 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct CreateProductReqData with 5 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = match _serde::de::SeqAccess::next_element::<
                            i32,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct CreateProductReqData with 5 elements",
                                    ),
                                );
                            }
                        };
                        let __field4 = match _serde::de::SeqAccess::next_element::<
                            String,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        4usize,
                                        &"struct CreateProductReqData with 5 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(CreateProductReqData {
                            name: __field0,
                            price: __field1,
                            picture: __field2,
                            count: __field3,
                            ext: __field4,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<f64> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<String> = _serde::__private::None;
                        let mut __field3: _serde::__private::Option<i32> = _serde::__private::None;
                        let mut __field4: _serde::__private::Option<String> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("price"),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<f64>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "picture",
                                            ),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("count"),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<i32>(&mut __map)?,
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::__private::Option::is_some(&__field4) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("ext"),
                                        );
                                    }
                                    __field4 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<String>(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("name")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("price")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("picture")?
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("count")?
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::__private::Some(__field4) => __field4,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("ext")?
                            }
                        };
                        _serde::__private::Ok(CreateProductReqData {
                            name: __field0,
                            price: __field1,
                            picture: __field2,
                            count: __field3,
                            ext: __field4,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "name",
                    "price",
                    "picture",
                    "count",
                    "ext",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "CreateProductReqData",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<CreateProductReqData>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl utoipa::__dev::ComposeSchema for CreateProductReqData {
        fn compose(
            mut generics: Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>>,
        ) -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
            {
                let mut object = utoipa::openapi::ObjectBuilder::new();
                object = object
                    .property(
                        "name",
                        utoipa::openapi::ObjectBuilder::new()
                            .schema_type(
                                utoipa::openapi::schema::SchemaType::new(
                                    utoipa::openapi::schema::Type::String,
                                ),
                            ),
                    )
                    .required("name");
                object = object
                    .property(
                        "price",
                        utoipa::openapi::ObjectBuilder::new()
                            .schema_type(
                                utoipa::openapi::schema::SchemaType::new(
                                    utoipa::openapi::schema::Type::Number,
                                ),
                            )
                            .format(
                                Some(
                                    utoipa::openapi::schema::SchemaFormat::KnownFormat(
                                        utoipa::openapi::schema::KnownFormat::Double,
                                    ),
                                ),
                            ),
                    )
                    .required("price");
                object = object
                    .property(
                        "picture",
                        utoipa::openapi::ObjectBuilder::new()
                            .schema_type(
                                utoipa::openapi::schema::SchemaType::new(
                                    utoipa::openapi::schema::Type::String,
                                ),
                            ),
                    )
                    .required("picture");
                object = object
                    .property(
                        "count",
                        utoipa::openapi::ObjectBuilder::new()
                            .schema_type(
                                utoipa::openapi::schema::SchemaType::new(
                                    utoipa::openapi::schema::Type::Integer,
                                ),
                            )
                            .format(
                                Some(
                                    utoipa::openapi::schema::SchemaFormat::KnownFormat(
                                        utoipa::openapi::schema::KnownFormat::Int32,
                                    ),
                                ),
                            ),
                    )
                    .required("count");
                object = object
                    .property(
                        "ext",
                        utoipa::openapi::ObjectBuilder::new()
                            .schema_type(
                                utoipa::openapi::schema::SchemaType::new(
                                    utoipa::openapi::schema::Type::String,
                                ),
                            ),
                    )
                    .required("ext");
                object
            }
                .into()
        }
    }
    impl utoipa::ToSchema for CreateProductReqData {
        fn name() -> std::borrow::Cow<'static, str> {
            std::borrow::Cow::Borrowed("CreateProductReqData")
        }
        fn schemas(
            schemas: &mut Vec<
                (String, utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>),
            >,
        ) {
            schemas.extend([]);
        }
    }
    impl From<ProductEntity> for CreateProductReqData {
        fn from(src: ProductEntity) -> Self {
            Self {
                name: src.name,
                price: src.price,
                picture: src.picture,
                count: src.count,
                ext: src.ext,
            }
        }
    }
    impl utoipa::__dev::ComposeSchema for ProductEntity {
        fn compose(
            mut generics: Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>>,
        ) -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
            {
                let mut object = utoipa::openapi::ObjectBuilder::new();
                object = object
                    .property(
                        "id",
                        utoipa::openapi::ObjectBuilder::new()
                            .schema_type({
                                use std::iter::FromIterator;
                                utoipa::openapi::schema::SchemaType::from_iter([
                                    utoipa::openapi::schema::Type::Integer,
                                    utoipa::openapi::schema::Type::Null,
                                ])
                            })
                            .format(
                                Some(
                                    utoipa::openapi::schema::SchemaFormat::KnownFormat(
                                        utoipa::openapi::schema::KnownFormat::Int32,
                                    ),
                                ),
                            ),
                    );
                object = object
                    .property(
                        "create_time",
                        utoipa::openapi::ObjectBuilder::new()
                            .schema_type(
                                utoipa::openapi::schema::SchemaType::new(
                                    utoipa::openapi::schema::Type::String,
                                ),
                            ),
                    )
                    .required("create_time");
                object = object
                    .property(
                        "update_time",
                        utoipa::openapi::ObjectBuilder::new()
                            .schema_type(
                                utoipa::openapi::schema::SchemaType::new(
                                    utoipa::openapi::schema::Type::String,
                                ),
                            ),
                    )
                    .required("update_time");
                object = object
                    .property(
                        "name",
                        utoipa::openapi::ObjectBuilder::new()
                            .schema_type(
                                utoipa::openapi::schema::SchemaType::new(
                                    utoipa::openapi::schema::Type::String,
                                ),
                            ),
                    )
                    .required("name");
                object = object
                    .property(
                        "status",
                        utoipa::openapi::ObjectBuilder::new()
                            .schema_type(
                                utoipa::openapi::schema::SchemaType::new(
                                    utoipa::openapi::schema::Type::Integer,
                                ),
                            )
                            .format(
                                Some(
                                    utoipa::openapi::schema::SchemaFormat::KnownFormat(
                                        utoipa::openapi::schema::KnownFormat::Int32,
                                    ),
                                ),
                            )
                            .minimum(Some(0f64)),
                    )
                    .required("status");
                object = object
                    .property(
                        "price",
                        utoipa::openapi::ObjectBuilder::new()
                            .schema_type(
                                utoipa::openapi::schema::SchemaType::new(
                                    utoipa::openapi::schema::Type::Number,
                                ),
                            )
                            .format(
                                Some(
                                    utoipa::openapi::schema::SchemaFormat::KnownFormat(
                                        utoipa::openapi::schema::KnownFormat::Double,
                                    ),
                                ),
                            ),
                    )
                    .required("price");
                object = object
                    .property(
                        "picture",
                        utoipa::openapi::ObjectBuilder::new()
                            .schema_type(
                                utoipa::openapi::schema::SchemaType::new(
                                    utoipa::openapi::schema::Type::String,
                                ),
                            ),
                    )
                    .required("picture");
                object = object
                    .property(
                        "count",
                        utoipa::openapi::ObjectBuilder::new()
                            .schema_type(
                                utoipa::openapi::schema::SchemaType::new(
                                    utoipa::openapi::schema::Type::Integer,
                                ),
                            )
                            .format(
                                Some(
                                    utoipa::openapi::schema::SchemaFormat::KnownFormat(
                                        utoipa::openapi::schema::KnownFormat::Int32,
                                    ),
                                ),
                            ),
                    )
                    .required("count");
                object = object
                    .property(
                        "ext",
                        utoipa::openapi::ObjectBuilder::new()
                            .schema_type(
                                utoipa::openapi::schema::SchemaType::new(
                                    utoipa::openapi::schema::Type::String,
                                ),
                            ),
                    )
                    .required("ext");
                object
            }
                .into()
        }
    }
    impl utoipa::ToSchema for ProductEntity {
        fn name() -> std::borrow::Cow<'static, str> {
            std::borrow::Cow::Borrowed("ProductEntity")
        }
        fn schemas(
            schemas: &mut Vec<
                (String, utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>),
            >,
        ) {
            schemas.extend([]);
        }
    }
    #[pick(
        arg(
            ident = PqProductItem,
            fields(name),
            derive(Clone, Debug, Serialize, Deserialize, ToSchema),
            forward_attrs(),
        ),
    )]
    pub struct UpdateProductReqData {
        pub name: Option<String>,
        pub status: Option<u8>,
        pub price: Option<f64>,
        pub picture: Option<String>,
        pub count: Option<i32>,
        pub ext: Option<String>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for UpdateProductReqData {
        #[inline]
        fn clone(&self) -> UpdateProductReqData {
            UpdateProductReqData {
                name: ::core::clone::Clone::clone(&self.name),
                status: ::core::clone::Clone::clone(&self.status),
                price: ::core::clone::Clone::clone(&self.price),
                picture: ::core::clone::Clone::clone(&self.picture),
                count: ::core::clone::Clone::clone(&self.count),
                ext: ::core::clone::Clone::clone(&self.ext),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for UpdateProductReqData {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            let names: &'static _ = &[
                "name",
                "status",
                "price",
                "picture",
                "count",
                "ext",
            ];
            let values: &[&dyn ::core::fmt::Debug] = &[
                &self.name,
                &self.status,
                &self.price,
                &self.picture,
                &self.count,
                &&self.ext,
            ];
            ::core::fmt::Formatter::debug_struct_fields_finish(
                f,
                "UpdateProductReqData",
                names,
                values,
            )
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for UpdateProductReqData {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "UpdateProductReqData",
                    false as usize + 1 + 1 + 1 + 1 + 1 + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "name",
                    &self.name,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "status",
                    &self.status,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "price",
                    &self.price,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "picture",
                    &self.picture,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "count",
                    &self.count,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "ext",
                    &self.ext,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for UpdateProductReqData {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __field1,
                    __field2,
                    __field3,
                    __field4,
                    __field5,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            1u64 => _serde::__private::Ok(__Field::__field1),
                            2u64 => _serde::__private::Ok(__Field::__field2),
                            3u64 => _serde::__private::Ok(__Field::__field3),
                            4u64 => _serde::__private::Ok(__Field::__field4),
                            5u64 => _serde::__private::Ok(__Field::__field5),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "name" => _serde::__private::Ok(__Field::__field0),
                            "status" => _serde::__private::Ok(__Field::__field1),
                            "price" => _serde::__private::Ok(__Field::__field2),
                            "picture" => _serde::__private::Ok(__Field::__field3),
                            "count" => _serde::__private::Ok(__Field::__field4),
                            "ext" => _serde::__private::Ok(__Field::__field5),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"name" => _serde::__private::Ok(__Field::__field0),
                            b"status" => _serde::__private::Ok(__Field::__field1),
                            b"price" => _serde::__private::Ok(__Field::__field2),
                            b"picture" => _serde::__private::Ok(__Field::__field3),
                            b"count" => _serde::__private::Ok(__Field::__field4),
                            b"ext" => _serde::__private::Ok(__Field::__field5),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<UpdateProductReqData>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = UpdateProductReqData;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct UpdateProductReqData",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            Option<String>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct UpdateProductReqData with 6 elements",
                                    ),
                                );
                            }
                        };
                        let __field1 = match _serde::de::SeqAccess::next_element::<
                            Option<u8>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        1usize,
                                        &"struct UpdateProductReqData with 6 elements",
                                    ),
                                );
                            }
                        };
                        let __field2 = match _serde::de::SeqAccess::next_element::<
                            Option<f64>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        2usize,
                                        &"struct UpdateProductReqData with 6 elements",
                                    ),
                                );
                            }
                        };
                        let __field3 = match _serde::de::SeqAccess::next_element::<
                            Option<String>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        3usize,
                                        &"struct UpdateProductReqData with 6 elements",
                                    ),
                                );
                            }
                        };
                        let __field4 = match _serde::de::SeqAccess::next_element::<
                            Option<i32>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        4usize,
                                        &"struct UpdateProductReqData with 6 elements",
                                    ),
                                );
                            }
                        };
                        let __field5 = match _serde::de::SeqAccess::next_element::<
                            Option<String>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        5usize,
                                        &"struct UpdateProductReqData with 6 elements",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(UpdateProductReqData {
                            name: __field0,
                            status: __field1,
                            price: __field2,
                            picture: __field3,
                            count: __field4,
                            ext: __field5,
                        })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<Option<String>> = _serde::__private::None;
                        let mut __field1: _serde::__private::Option<Option<u8>> = _serde::__private::None;
                        let mut __field2: _serde::__private::Option<Option<f64>> = _serde::__private::None;
                        let mut __field3: _serde::__private::Option<Option<String>> = _serde::__private::None;
                        let mut __field4: _serde::__private::Option<Option<i32>> = _serde::__private::None;
                        let mut __field5: _serde::__private::Option<Option<String>> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<String>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field1 => {
                                    if _serde::__private::Option::is_some(&__field1) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("status"),
                                        );
                                    }
                                    __field1 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<Option<u8>>(&mut __map)?,
                                    );
                                }
                                __Field::__field2 => {
                                    if _serde::__private::Option::is_some(&__field2) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("price"),
                                        );
                                    }
                                    __field2 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<f64>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field3 => {
                                    if _serde::__private::Option::is_some(&__field3) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field(
                                                "picture",
                                            ),
                                        );
                                    }
                                    __field3 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<String>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field4 => {
                                    if _serde::__private::Option::is_some(&__field4) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("count"),
                                        );
                                    }
                                    __field4 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<i32>,
                                        >(&mut __map)?,
                                    );
                                }
                                __Field::__field5 => {
                                    if _serde::__private::Option::is_some(&__field5) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("ext"),
                                        );
                                    }
                                    __field5 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<String>,
                                        >(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("name")?
                            }
                        };
                        let __field1 = match __field1 {
                            _serde::__private::Some(__field1) => __field1,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("status")?
                            }
                        };
                        let __field2 = match __field2 {
                            _serde::__private::Some(__field2) => __field2,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("price")?
                            }
                        };
                        let __field3 = match __field3 {
                            _serde::__private::Some(__field3) => __field3,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("picture")?
                            }
                        };
                        let __field4 = match __field4 {
                            _serde::__private::Some(__field4) => __field4,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("count")?
                            }
                        };
                        let __field5 = match __field5 {
                            _serde::__private::Some(__field5) => __field5,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("ext")?
                            }
                        };
                        _serde::__private::Ok(UpdateProductReqData {
                            name: __field0,
                            status: __field1,
                            price: __field2,
                            picture: __field3,
                            count: __field4,
                            ext: __field5,
                        })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &[
                    "name",
                    "status",
                    "price",
                    "picture",
                    "count",
                    "ext",
                ];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "UpdateProductReqData",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<UpdateProductReqData>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    pub struct PqProductItem {
        pub name: Option<String>,
    }
    #[automatically_derived]
    impl ::core::clone::Clone for PqProductItem {
        #[inline]
        fn clone(&self) -> PqProductItem {
            PqProductItem {
                name: ::core::clone::Clone::clone(&self.name),
            }
        }
    }
    #[automatically_derived]
    impl ::core::fmt::Debug for PqProductItem {
        #[inline]
        fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
            ::core::fmt::Formatter::debug_struct_field1_finish(
                f,
                "PqProductItem",
                "name",
                &&self.name,
            )
        }
    }
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl _serde::Serialize for PqProductItem {
            fn serialize<__S>(
                &self,
                __serializer: __S,
            ) -> _serde::__private::Result<__S::Ok, __S::Error>
            where
                __S: _serde::Serializer,
            {
                let mut __serde_state = _serde::Serializer::serialize_struct(
                    __serializer,
                    "PqProductItem",
                    false as usize + 1,
                )?;
                _serde::ser::SerializeStruct::serialize_field(
                    &mut __serde_state,
                    "name",
                    &self.name,
                )?;
                _serde::ser::SerializeStruct::end(__serde_state)
            }
        }
    };
    #[doc(hidden)]
    #[allow(non_upper_case_globals, unused_attributes, unused_qualifications)]
    const _: () = {
        #[allow(unused_extern_crates, clippy::useless_attribute)]
        extern crate serde as _serde;
        #[automatically_derived]
        impl<'de> _serde::Deserialize<'de> for PqProductItem {
            fn deserialize<__D>(
                __deserializer: __D,
            ) -> _serde::__private::Result<Self, __D::Error>
            where
                __D: _serde::Deserializer<'de>,
            {
                #[allow(non_camel_case_types)]
                #[doc(hidden)]
                enum __Field {
                    __field0,
                    __ignore,
                }
                #[doc(hidden)]
                struct __FieldVisitor;
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __FieldVisitor {
                    type Value = __Field;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "field identifier",
                        )
                    }
                    fn visit_u64<__E>(
                        self,
                        __value: u64,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            0u64 => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_str<__E>(
                        self,
                        __value: &str,
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            "name" => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                    fn visit_bytes<__E>(
                        self,
                        __value: &[u8],
                    ) -> _serde::__private::Result<Self::Value, __E>
                    where
                        __E: _serde::de::Error,
                    {
                        match __value {
                            b"name" => _serde::__private::Ok(__Field::__field0),
                            _ => _serde::__private::Ok(__Field::__ignore),
                        }
                    }
                }
                #[automatically_derived]
                impl<'de> _serde::Deserialize<'de> for __Field {
                    #[inline]
                    fn deserialize<__D>(
                        __deserializer: __D,
                    ) -> _serde::__private::Result<Self, __D::Error>
                    where
                        __D: _serde::Deserializer<'de>,
                    {
                        _serde::Deserializer::deserialize_identifier(
                            __deserializer,
                            __FieldVisitor,
                        )
                    }
                }
                #[doc(hidden)]
                struct __Visitor<'de> {
                    marker: _serde::__private::PhantomData<PqProductItem>,
                    lifetime: _serde::__private::PhantomData<&'de ()>,
                }
                #[automatically_derived]
                impl<'de> _serde::de::Visitor<'de> for __Visitor<'de> {
                    type Value = PqProductItem;
                    fn expecting(
                        &self,
                        __formatter: &mut _serde::__private::Formatter,
                    ) -> _serde::__private::fmt::Result {
                        _serde::__private::Formatter::write_str(
                            __formatter,
                            "struct PqProductItem",
                        )
                    }
                    #[inline]
                    fn visit_seq<__A>(
                        self,
                        mut __seq: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::SeqAccess<'de>,
                    {
                        let __field0 = match _serde::de::SeqAccess::next_element::<
                            Option<String>,
                        >(&mut __seq)? {
                            _serde::__private::Some(__value) => __value,
                            _serde::__private::None => {
                                return _serde::__private::Err(
                                    _serde::de::Error::invalid_length(
                                        0usize,
                                        &"struct PqProductItem with 1 element",
                                    ),
                                );
                            }
                        };
                        _serde::__private::Ok(PqProductItem { name: __field0 })
                    }
                    #[inline]
                    fn visit_map<__A>(
                        self,
                        mut __map: __A,
                    ) -> _serde::__private::Result<Self::Value, __A::Error>
                    where
                        __A: _serde::de::MapAccess<'de>,
                    {
                        let mut __field0: _serde::__private::Option<Option<String>> = _serde::__private::None;
                        while let _serde::__private::Some(__key) = _serde::de::MapAccess::next_key::<
                            __Field,
                        >(&mut __map)? {
                            match __key {
                                __Field::__field0 => {
                                    if _serde::__private::Option::is_some(&__field0) {
                                        return _serde::__private::Err(
                                            <__A::Error as _serde::de::Error>::duplicate_field("name"),
                                        );
                                    }
                                    __field0 = _serde::__private::Some(
                                        _serde::de::MapAccess::next_value::<
                                            Option<String>,
                                        >(&mut __map)?,
                                    );
                                }
                                _ => {
                                    let _ = _serde::de::MapAccess::next_value::<
                                        _serde::de::IgnoredAny,
                                    >(&mut __map)?;
                                }
                            }
                        }
                        let __field0 = match __field0 {
                            _serde::__private::Some(__field0) => __field0,
                            _serde::__private::None => {
                                _serde::__private::de::missing_field("name")?
                            }
                        };
                        _serde::__private::Ok(PqProductItem { name: __field0 })
                    }
                }
                #[doc(hidden)]
                const FIELDS: &'static [&'static str] = &["name"];
                _serde::Deserializer::deserialize_struct(
                    __deserializer,
                    "PqProductItem",
                    FIELDS,
                    __Visitor {
                        marker: _serde::__private::PhantomData::<PqProductItem>,
                        lifetime: _serde::__private::PhantomData,
                    },
                )
            }
        }
    };
    impl utoipa::__dev::ComposeSchema for PqProductItem {
        fn compose(
            mut generics: Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>>,
        ) -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
            {
                let mut object = utoipa::openapi::ObjectBuilder::new();
                object = object
                    .property(
                        "name",
                        utoipa::openapi::ObjectBuilder::new()
                            .schema_type({
                                use std::iter::FromIterator;
                                utoipa::openapi::schema::SchemaType::from_iter([
                                    utoipa::openapi::schema::Type::String,
                                    utoipa::openapi::schema::Type::Null,
                                ])
                            }),
                    );
                object
            }
                .into()
        }
    }
    impl utoipa::ToSchema for PqProductItem {
        fn name() -> std::borrow::Cow<'static, str> {
            std::borrow::Cow::Borrowed("PqProductItem")
        }
        fn schemas(
            schemas: &mut Vec<
                (String, utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>),
            >,
        ) {
            schemas.extend([]);
        }
    }
    impl From<UpdateProductReqData> for PqProductItem {
        fn from(src: UpdateProductReqData) -> Self {
            Self { name: src.name }
        }
    }
    impl utoipa::__dev::ComposeSchema for UpdateProductReqData {
        fn compose(
            mut generics: Vec<utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>>,
        ) -> utoipa::openapi::RefOr<utoipa::openapi::schema::Schema> {
            {
                let mut object = utoipa::openapi::ObjectBuilder::new();
                object = object
                    .property(
                        "name",
                        utoipa::openapi::ObjectBuilder::new()
                            .schema_type({
                                use std::iter::FromIterator;
                                utoipa::openapi::schema::SchemaType::from_iter([
                                    utoipa::openapi::schema::Type::String,
                                    utoipa::openapi::schema::Type::Null,
                                ])
                            }),
                    );
                object = object
                    .property(
                        "status",
                        utoipa::openapi::ObjectBuilder::new()
                            .schema_type({
                                use std::iter::FromIterator;
                                utoipa::openapi::schema::SchemaType::from_iter([
                                    utoipa::openapi::schema::Type::Integer,
                                    utoipa::openapi::schema::Type::Null,
                                ])
                            })
                            .format(
                                Some(
                                    utoipa::openapi::schema::SchemaFormat::KnownFormat(
                                        utoipa::openapi::schema::KnownFormat::Int32,
                                    ),
                                ),
                            )
                            .minimum(Some(0f64)),
                    );
                object = object
                    .property(
                        "price",
                        utoipa::openapi::ObjectBuilder::new()
                            .schema_type({
                                use std::iter::FromIterator;
                                utoipa::openapi::schema::SchemaType::from_iter([
                                    utoipa::openapi::schema::Type::Number,
                                    utoipa::openapi::schema::Type::Null,
                                ])
                            })
                            .format(
                                Some(
                                    utoipa::openapi::schema::SchemaFormat::KnownFormat(
                                        utoipa::openapi::schema::KnownFormat::Double,
                                    ),
                                ),
                            ),
                    );
                object = object
                    .property(
                        "picture",
                        utoipa::openapi::ObjectBuilder::new()
                            .schema_type({
                                use std::iter::FromIterator;
                                utoipa::openapi::schema::SchemaType::from_iter([
                                    utoipa::openapi::schema::Type::String,
                                    utoipa::openapi::schema::Type::Null,
                                ])
                            }),
                    );
                object = object
                    .property(
                        "count",
                        utoipa::openapi::ObjectBuilder::new()
                            .schema_type({
                                use std::iter::FromIterator;
                                utoipa::openapi::schema::SchemaType::from_iter([
                                    utoipa::openapi::schema::Type::Integer,
                                    utoipa::openapi::schema::Type::Null,
                                ])
                            })
                            .format(
                                Some(
                                    utoipa::openapi::schema::SchemaFormat::KnownFormat(
                                        utoipa::openapi::schema::KnownFormat::Int32,
                                    ),
                                ),
                            ),
                    );
                object = object
                    .property(
                        "ext",
                        utoipa::openapi::ObjectBuilder::new()
                            .schema_type({
                                use std::iter::FromIterator;
                                utoipa::openapi::schema::SchemaType::from_iter([
                                    utoipa::openapi::schema::Type::String,
                                    utoipa::openapi::schema::Type::Null,
                                ])
                            }),
                    );
                object
            }
                .into()
        }
    }
    impl utoipa::ToSchema for UpdateProductReqData {
        fn name() -> std::borrow::Cow<'static, str> {
            std::borrow::Cow::Borrowed("UpdateProductReqData")
        }
        fn schemas(
            schemas: &mut Vec<
                (String, utoipa::openapi::RefOr<utoipa::openapi::schema::Schema>),
            >,
        ) {
            schemas.extend([]);
        }
    }
    impl Default for ProductEntity {
        fn default() -> Self {
            Self {
                id: None,
                create_time: "".to_string(),
                update_time: "".to_string(),
                name: "".to_string(),
                status: 0,
                price: 0.0,
                picture: "".to_string(),
                count: 0,
                ext: "".to_string(),
            }
        }
    }
    impl From<CreateProductReqData> for ProductEntity {
        fn from(data: CreateProductReqData) -> Self {
            Self {
                id: None,
                create_time: get_current_time_fmt(),
                update_time: get_current_time_fmt(),
                name: data.name,
                status: Status::DEACTIVE as u8,
                price: data.price,
                picture: data.picture,
                count: data.count,
                ext: data.ext,
            }
        }
    }
    impl ProductEntity {
        pub async fn insert_batch(
            executor: &dyn ::rbatis::executor::Executor,
            tables: &[ProductEntity],
            batch_size: u64,
        ) -> std::result::Result<::rbatis::rbdc::db::ExecResult, ::rbatis::rbdc::Error> {
            pub trait ColumnSet {
                /// take `vec![Table{"id":1}]` columns
                fn column_sets(&self) -> rbs::Value;
            }
            impl ColumnSet for rbs::Value {
                fn column_sets(&self) -> rbs::Value {
                    let len = self.len();
                    let mut column_set = std::collections::HashSet::with_capacity(len);
                    if let Some(array) = self.as_array() {
                        for item in array {
                            for (k, v) in &item {
                                if (*v) != rbs::Value::Null {
                                    column_set.insert(k);
                                }
                            }
                        }
                    }
                    let mut columns = rbs::Value::Array(::alloc::vec::Vec::new());
                    if len > 0 {
                        let table = &self[0];
                        let mut column_datas = Vec::with_capacity(table.len());
                        for (column, _) in table {
                            if column_set.contains(&column) {
                                column_datas.push(column);
                            }
                        }
                        columns = rbs::Value::from(column_datas);
                    }
                    columns
                }
            }
            pub async fn insert_batch(
                executor: &dyn ::rbatis::executor::Executor,
                tables: &[ProductEntity],
                table_name: &str,
            ) -> std::result::Result<
                ::rbatis::rbdc::db::ExecResult,
                ::rbatis::rbdc::Error,
            > {
                let mut rb_arg_map = rbs::value::map::ValueMap::with_capacity(2usize);
                rb_arg_map.insert("tables".to_string().into(), rbs::to_value(tables)?);
                rb_arg_map
                    .insert("table_name".to_string().into(), rbs::to_value(table_name)?);
                {}
                use rbatis::executor::RBatisRef;
                let driver_type = executor.rb_ref().driver_type()?;
                use rbatis::rbatis_codegen;
                pub fn do_py_sql(
                    mut arg: rbs::Value,
                    _tag: char,
                ) -> (String, Vec<rbs::Value>) {
                    use rbatis_codegen::ops::*;
                    let mut sql = String::with_capacity(1072usize);
                    let mut args = Vec::with_capacity(1usize);
                    sql.push_str(
                        &::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "insert into {0} ",
                                    &{ &arg["table_name"] }.string(),
                                ),
                            );
                            res
                        }),
                    );
                    sql.push_str(
                        &{
                            let mut sql = String::with_capacity(42usize);
                            if arg["columns"] == rbs::Value::Null {
                                arg.insert(
                                    rbs::Value::String("columns".to_string()),
                                    rbs::Value::Null,
                                );
                            }
                            arg["columns"] = rbs::to_value({
                                    (&arg["tables"]).column_sets()
                                })
                                .unwrap_or_default();
                            for (ref idx, table) in { &arg["tables"] } {
                                if { (idx).op_eq(&0i64) }.to_owned().into() {
                                    sql.push_str("(");
                                    sql.push_str(
                                        &{
                                            let mut sql = String::with_capacity(7usize);
                                            for (ref index, v) in { &arg["columns"] } {
                                                sql.push_str(
                                                    &::alloc::__export::must_use({
                                                        let res = ::alloc::fmt::format(
                                                            format_args!("{0},", &{ v }.string()),
                                                        );
                                                        res
                                                    }),
                                                );
                                            }
                                            sql = sql
                                                .trim_start_matches(",")
                                                .trim_end_matches(",")
                                                .to_string();
                                            sql
                                        },
                                    );
                                    sql.push_str(") VALUES ");
                                }
                                sql.push_str("(");
                                sql.push_str(
                                    &{
                                        let mut sql = String::with_capacity(14usize);
                                        for (ref index, v) in { &arg["columns"] } {
                                            args.push(rbs::to_value({ &table[v] }).unwrap_or_default());
                                            sql.push_str("?,");
                                        }
                                        sql = sql
                                            .trim_start_matches(",")
                                            .trim_end_matches(",")
                                            .to_string();
                                        sql
                                    },
                                );
                                sql.push_str("),");
                            }
                            sql = sql
                                .trim_start_matches(",")
                                .trim_end_matches(",")
                                .to_string();
                            sql
                        },
                    );
                    return (sql, args);
                }
                let (mut sql, rb_args) = do_py_sql(rbs::Value::Map(rb_arg_map), '?');
                use rbatis::executor::Executor;
                executor.exec(&sql, rb_args).await
            }
            if tables.is_empty() {
                return Err(
                    ::rbatis::rbdc::Error::from(
                        "insert can not insert empty array tables!",
                    ),
                );
            }
            pub fn snake_name() -> String {
                "product_entity".to_string()
            }
            let mut table_name = "product".to_string();
            if table_name.is_empty() {
                table_name = snake_name();
            }
            let mut result = ::rbatis::rbdc::db::ExecResult {
                rows_affected: 0,
                last_insert_id: rbs::Value::Null,
            };
            let ranges = ::rbatis::plugin::Page::<
                (),
            >::make_ranges(tables.len() as u64, batch_size);
            for (offset, limit) in ranges {
                let exec_result = insert_batch(
                        executor,
                        &tables[offset as usize..limit as usize],
                        table_name.as_str(),
                    )
                    .await?;
                result.rows_affected += exec_result.rows_affected;
                result.last_insert_id = exec_result.last_insert_id;
            }
            Ok(result)
        }
        pub async fn insert(
            executor: &dyn ::rbatis::executor::Executor,
            table: &ProductEntity,
        ) -> std::result::Result<::rbatis::rbdc::db::ExecResult, ::rbatis::rbdc::Error> {
            <ProductEntity>::insert_batch(executor, std::slice::from_ref(table), 1).await
        }
    }
    impl ProductEntity {
        pub async fn select_all(
            executor: &dyn ::rbatis::executor::Executor,
        ) -> std::result::Result<Vec<ProductEntity>, ::rbatis::rbdc::Error> {
            pub async fn select_all(
                executor: &dyn ::rbatis::executor::Executor,
                table_column: &str,
                table_name: &str,
            ) -> std::result::Result<Vec<ProductEntity>, ::rbatis::rbdc::Error> {
                let mut rb_arg_map = rbs::value::map::ValueMap::with_capacity(2usize);
                rb_arg_map
                    .insert(
                        "table_column".to_string().into(),
                        rbs::to_value(table_column)?,
                    );
                rb_arg_map
                    .insert("table_name".to_string().into(), rbs::to_value(table_name)?);
                {}
                use rbatis::executor::RBatisRef;
                let driver_type = executor.rb_ref().driver_type()?;
                use rbatis::rbatis_codegen;
                pub fn do_py_sql(
                    mut arg: rbs::Value,
                    _tag: char,
                ) -> (String, Vec<rbs::Value>) {
                    use rbatis_codegen::ops::*;
                    let mut sql = String::with_capacity(1044usize);
                    let mut args = Vec::with_capacity(0usize);
                    sql.push_str(
                        &::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "select {0} from {1} ",
                                    &{ &arg["table_column"] }.string(),
                                    &{ &arg["table_name"] }.string(),
                                ),
                            );
                            res
                        }),
                    );
                    return (sql, args);
                }
                let (mut sql, rb_args) = do_py_sql(rbs::Value::Map(rb_arg_map), '?');
                use rbatis::executor::Executor;
                let r = executor.query(&sql, rb_args).await?;
                rbatis::decode::decode(r)
            }
            let mut table_column = "*".to_string();
            let mut table_name = String::new();
            table_name = "product".to_string();
            pub fn snake_name() -> String {
                "product_entity".to_string()
            }
            if table_name.is_empty() {
                table_name = snake_name();
            }
            select_all(executor, &table_column, &table_name).await
        }
    }
    impl ProductEntity {
        pub async fn select_by_column<V: serde::Serialize>(
            executor: &dyn ::rbatis::executor::Executor,
            column: &str,
            column_value: V,
        ) -> std::result::Result<Vec<ProductEntity>, ::rbatis::rbdc::Error> {
            pub async fn select_by_column<V: serde::Serialize>(
                executor: &dyn ::rbatis::executor::Executor,
                table_column: &str,
                table_name: &str,
                column: &str,
                column_value: V,
            ) -> std::result::Result<Vec<ProductEntity>, ::rbatis::rbdc::Error> {
                let mut rb_arg_map = rbs::value::map::ValueMap::with_capacity(4usize);
                rb_arg_map
                    .insert(
                        "table_column".to_string().into(),
                        rbs::to_value(table_column)?,
                    );
                rb_arg_map
                    .insert("table_name".to_string().into(), rbs::to_value(table_name)?);
                rb_arg_map.insert("column".to_string().into(), rbs::to_value(column)?);
                rb_arg_map
                    .insert(
                        "column_value".to_string().into(),
                        rbs::to_value(column_value)?,
                    );
                {}
                use rbatis::executor::RBatisRef;
                let driver_type = executor.rb_ref().driver_type()?;
                use rbatis::rbatis_codegen;
                pub fn do_py_sql(
                    mut arg: rbs::Value,
                    _tag: char,
                ) -> (String, Vec<rbs::Value>) {
                    use rbatis_codegen::ops::*;
                    let mut sql = String::with_capacity(1080usize);
                    let mut args = Vec::with_capacity(1usize);
                    args.push(
                        rbs::to_value({ &arg["column_value"] }).unwrap_or_default(),
                    );
                    sql.push_str(
                        &::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "select {0} from {1}  where {2} = ?",
                                    &{ &arg["table_column"] }.string(),
                                    &{ &arg["table_name"] }.string(),
                                    &{ &arg["column"] }.string(),
                                ),
                            );
                            res
                        }),
                    );
                    return (sql, args);
                }
                let (mut sql, rb_args) = do_py_sql(rbs::Value::Map(rb_arg_map), '?');
                use rbatis::executor::Executor;
                let r = executor.query(&sql, rb_args).await?;
                rbatis::decode::decode(r)
            }
            let mut table_column = "*".to_string();
            let mut table_name = String::new();
            table_name = "product".to_string();
            pub fn snake_name() -> String {
                "product_entity".to_string()
            }
            if table_name.is_empty() {
                table_name = snake_name();
            }
            select_by_column(executor, &table_column, &table_name, column, column_value)
                .await
        }
    }
    impl ProductEntity {
        pub async fn select_in_column<V: serde::Serialize>(
            executor: &dyn ::rbatis::executor::Executor,
            column: &str,
            column_values: &[V],
        ) -> std::result::Result<Vec<ProductEntity>, ::rbatis::rbdc::Error> {
            pub async fn select_in_column<V: serde::Serialize>(
                executor: &dyn ::rbatis::executor::Executor,
                table_column: &str,
                table_name: &str,
                column: &str,
                column_values: &[V],
            ) -> std::result::Result<Vec<ProductEntity>, ::rbatis::rbdc::Error> {
                let mut rb_arg_map = rbs::value::map::ValueMap::with_capacity(4usize);
                rb_arg_map
                    .insert(
                        "table_column".to_string().into(),
                        rbs::to_value(table_column)?,
                    );
                rb_arg_map
                    .insert("table_name".to_string().into(), rbs::to_value(table_name)?);
                rb_arg_map.insert("column".to_string().into(), rbs::to_value(column)?);
                rb_arg_map
                    .insert(
                        "column_values".to_string().into(),
                        rbs::to_value(column_values)?,
                    );
                {}
                use rbatis::executor::RBatisRef;
                let driver_type = executor.rb_ref().driver_type()?;
                use rbatis::rbatis_codegen;
                pub fn do_py_sql(
                    mut arg: rbs::Value,
                    _tag: char,
                ) -> (String, Vec<rbs::Value>) {
                    use rbatis_codegen::ops::*;
                    let mut sql = String::with_capacity(1080usize);
                    let mut args = Vec::with_capacity(1usize);
                    sql.push_str(
                        &::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "select {0} from {1}  where {2} in (",
                                    &{ &arg["table_column"] }.string(),
                                    &{ &arg["table_name"] }.string(),
                                    &{ &arg["column"] }.string(),
                                ),
                            );
                            res
                        }),
                    );
                    sql.push_str(
                        &{
                            let mut sql = String::with_capacity(10usize);
                            for (ref index, item) in { &arg["column_values"] } {
                                args.push(rbs::to_value({ item }).unwrap_or_default());
                                sql.push_str("?,");
                            }
                            sql = sql
                                .trim_start_matches(",")
                                .trim_end_matches(",")
                                .to_string();
                            sql
                        },
                    );
                    sql.push_str(")");
                    return (sql, args);
                }
                let (mut sql, rb_args) = do_py_sql(rbs::Value::Map(rb_arg_map), '?');
                use rbatis::executor::Executor;
                let r = executor.query(&sql, rb_args).await?;
                rbatis::decode::decode(r)
            }
            let mut table_column = "*".to_string();
            let mut table_name = String::new();
            table_name = "product".to_string();
            pub fn snake_name() -> String {
                "product_entity".to_string()
            }
            if table_name.is_empty() {
                table_name = snake_name();
            }
            select_in_column(executor, &table_column, &table_name, column, column_values)
                .await
        }
    }
    impl ProductEntity {
        pub async fn update_by_column_value(
            executor: &dyn ::rbatis::executor::Executor,
            table: &ProductEntity,
            column: &str,
            column_value: &rbs::Value,
            skip_null: bool,
        ) -> std::result::Result<::rbatis::rbdc::db::ExecResult, ::rbatis::rbdc::Error> {
            if "`where ${column} = #{column_value}`".is_empty() {
                return Err(::rbatis::rbdc::Error::from("sql_where can't be empty!"));
            }
            pub async fn update_by_column_value(
                executor: &dyn ::rbatis::executor::Executor,
                table_name: String,
                table: &rbs::Value,
                skip_null: bool,
                column: &str,
                column_value: &rbs::Value,
                skip_null: bool,
            ) -> std::result::Result<
                ::rbatis::rbdc::db::ExecResult,
                ::rbatis::rbdc::Error,
            > {
                let mut rb_arg_map = rbs::value::map::ValueMap::with_capacity(6usize);
                rb_arg_map
                    .insert("table_name".to_string().into(), rbs::to_value(table_name)?);
                rb_arg_map.insert("table".to_string().into(), rbs::to_value(table)?);
                rb_arg_map
                    .insert("skip_null".to_string().into(), rbs::to_value(skip_null)?);
                rb_arg_map.insert("column".to_string().into(), rbs::to_value(column)?);
                rb_arg_map
                    .insert(
                        "column_value".to_string().into(),
                        rbs::to_value(column_value)?,
                    );
                rb_arg_map
                    .insert("skip_null".to_string().into(), rbs::to_value(skip_null)?);
                {}
                use rbatis::executor::RBatisRef;
                let driver_type = executor.rb_ref().driver_type()?;
                use rbatis::rbatis_codegen;
                pub fn do_py_sql(
                    mut arg: rbs::Value,
                    _tag: char,
                ) -> (String, Vec<rbs::Value>) {
                    use rbatis_codegen::ops::*;
                    let mut sql = String::with_capacity(1077usize);
                    let mut args = Vec::with_capacity(2usize);
                    sql.push_str(
                        &::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "update {0} set ",
                                    &{ &arg["table_name"] }.string(),
                                ),
                            );
                            res
                        }),
                    );
                    sql.push_str(
                        &{
                            let mut sql = String::with_capacity(12usize);
                            for (ref k, v) in { &arg["table"] } {
                                if { (k).op_eq(&&arg["column"]) }.to_owned().into() {
                                    continue;
                                }
                                if {
                                    bool::op_from((&arg["skip_null"]).op_eq(&true))
                                        && bool::op_from((v).op_eq(&rbs::Value::Null))
                                }
                                    .to_owned()
                                    .into()
                                {
                                    continue;
                                }
                                args.push(rbs::to_value({ v }).unwrap_or_default());
                                sql.push_str(
                                    &::alloc::__export::must_use({
                                        let res = ::alloc::fmt::format(
                                            format_args!("{0}=?,", &{ k }.string()),
                                        );
                                        res
                                    }),
                                );
                            }
                            sql = sql
                                .trim_start_matches(",")
                                .trim_end_matches(",")
                                .to_string();
                            sql
                        },
                    );
                    args.push(
                        rbs::to_value({ &arg["column_value"] }).unwrap_or_default(),
                    );
                    sql.push_str(
                        &::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!(" where {0} = ?", &{ &arg["column"] }.string()),
                            );
                            res
                        }),
                    );
                    return (sql, args);
                }
                let (mut sql, rb_args) = do_py_sql(rbs::Value::Map(rb_arg_map), '?');
                use rbatis::executor::Executor;
                executor.exec(&sql, rb_args).await
            }
            let mut table_name = String::new();
            table_name = "product".to_string();
            pub fn snake_name() -> String {
                "product_entity".to_string()
            }
            if table_name.is_empty() {
                table_name = snake_name();
            }
            let table = ::rbs::to_value(table).unwrap_or_default();
            update_by_column_value(
                    executor,
                    table_name,
                    &table,
                    true,
                    column,
                    column_value,
                    skip_null,
                )
                .await
        }
    }
    impl ProductEntity {
        ///  will skip null column
        pub async fn update_by_column(
            executor: &dyn ::rbatis::executor::Executor,
            table: &ProductEntity,
            column: &str,
        ) -> std::result::Result<::rbatis::rbdc::db::ExecResult, ::rbatis::rbdc::Error> {
            <ProductEntity>::update_by_column_skip(executor, table, column, true).await
        }
        ///will skip null column
        pub async fn update_by_column_batch(
            executor: &dyn ::rbatis::executor::Executor,
            tables: &[ProductEntity],
            column: &str,
            batch_size: u64,
        ) -> std::result::Result<::rbatis::rbdc::db::ExecResult, ::rbatis::rbdc::Error> {
            <ProductEntity>::update_by_column_batch_skip(
                    executor,
                    tables,
                    column,
                    batch_size,
                    true,
                )
                .await
        }
        pub async fn update_by_column_skip(
            executor: &dyn ::rbatis::executor::Executor,
            table: &ProductEntity,
            column: &str,
            skip_null: bool,
        ) -> std::result::Result<::rbatis::rbdc::db::ExecResult, ::rbatis::rbdc::Error> {
            let columns = ::rbs::to_value(table).unwrap_or_default();
            let column_value = &columns[column];
            <ProductEntity>::update_by_column_value(
                    executor,
                    table,
                    column,
                    column_value,
                    skip_null,
                )
                .await
        }
        pub async fn update_by_column_batch_skip(
            executor: &dyn ::rbatis::executor::Executor,
            tables: &[ProductEntity],
            column: &str,
            batch_size: u64,
            skip_null: bool,
        ) -> std::result::Result<::rbatis::rbdc::db::ExecResult, ::rbatis::rbdc::Error> {
            let mut rows_affected = 0;
            let ranges = ::rbatis::plugin::Page::<
                (),
            >::make_ranges(tables.len() as u64, batch_size);
            for (offset, limit) in ranges {
                for table in &tables[offset as usize..limit as usize] {
                    rows_affected
                        += <ProductEntity>::update_by_column_skip(
                                executor,
                                table,
                                column,
                                skip_null,
                            )
                            .await?
                            .rows_affected;
                }
            }
            Ok(::rbatis::rbdc::db::ExecResult {
                rows_affected: rows_affected,
                last_insert_id: rbs::Value::Null,
            })
        }
    }
    impl ProductEntity {
        pub async fn delete_by_column<V: serde::Serialize>(
            executor: &dyn ::rbatis::executor::Executor,
            column: &str,
            column_value: V,
        ) -> std::result::Result<::rbatis::rbdc::db::ExecResult, ::rbatis::rbdc::Error> {
            if "`where ${column} = #{column_value}`".is_empty() {
                return Err(::rbatis::rbdc::Error::from("sql_where can't be empty!"));
            }
            pub async fn delete_by_column<V: serde::Serialize>(
                executor: &dyn ::rbatis::executor::Executor,
                table_name: String,
                column: &str,
                column_value: V,
            ) -> std::result::Result<
                ::rbatis::rbdc::db::ExecResult,
                ::rbatis::rbdc::Error,
            > {
                let mut rb_arg_map = rbs::value::map::ValueMap::with_capacity(3usize);
                rb_arg_map
                    .insert("table_name".to_string().into(), rbs::to_value(table_name)?);
                rb_arg_map.insert("column".to_string().into(), rbs::to_value(column)?);
                rb_arg_map
                    .insert(
                        "column_value".to_string().into(),
                        rbs::to_value(column_value)?,
                    );
                {}
                use rbatis::executor::RBatisRef;
                let driver_type = executor.rb_ref().driver_type()?;
                use rbatis::rbatis_codegen;
                pub fn do_py_sql(
                    mut arg: rbs::Value,
                    _tag: char,
                ) -> (String, Vec<rbs::Value>) {
                    use rbatis_codegen::ops::*;
                    let mut sql = String::with_capacity(1063usize);
                    let mut args = Vec::with_capacity(1usize);
                    args.push(
                        rbs::to_value({ &arg["column_value"] }).unwrap_or_default(),
                    );
                    sql.push_str(
                        &::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "delete from {0} where {1} = ?",
                                    &{ &arg["table_name"] }.string(),
                                    &{ &arg["column"] }.string(),
                                ),
                            );
                            res
                        }),
                    );
                    return (sql, args);
                }
                let (mut sql, rb_args) = do_py_sql(rbs::Value::Map(rb_arg_map), '?');
                use rbatis::executor::Executor;
                executor.exec(&sql, rb_args).await
            }
            let mut table_name = String::new();
            table_name = "product".to_string();
            pub fn snake_name() -> String {
                "product_entity".to_string()
            }
            if table_name.is_empty() {
                table_name = snake_name();
            }
            delete_by_column(executor, table_name, column, column_value).await
        }
    }
    impl ProductEntity {
        pub async fn delete_in_column<V: serde::Serialize>(
            executor: &dyn ::rbatis::executor::Executor,
            column: &str,
            column_values: &[V],
        ) -> std::result::Result<::rbatis::rbdc::db::ExecResult, ::rbatis::rbdc::Error> {
            if "`where ${column} in (`
          trim ',': for _,item in column_values:
             #{item},
          `)`"
                .is_empty()
            {
                return Err(::rbatis::rbdc::Error::from("sql_where can't be empty!"));
            }
            pub async fn delete_in_column<V: serde::Serialize>(
                executor: &dyn ::rbatis::executor::Executor,
                table_name: String,
                column: &str,
                column_values: &[V],
            ) -> std::result::Result<
                ::rbatis::rbdc::db::ExecResult,
                ::rbatis::rbdc::Error,
            > {
                let mut rb_arg_map = rbs::value::map::ValueMap::with_capacity(3usize);
                rb_arg_map
                    .insert("table_name".to_string().into(), rbs::to_value(table_name)?);
                rb_arg_map.insert("column".to_string().into(), rbs::to_value(column)?);
                rb_arg_map
                    .insert(
                        "column_values".to_string().into(),
                        rbs::to_value(column_values)?,
                    );
                {}
                use rbatis::executor::RBatisRef;
                let driver_type = executor.rb_ref().driver_type()?;
                use rbatis::rbatis_codegen;
                pub fn do_py_sql(
                    mut arg: rbs::Value,
                    _tag: char,
                ) -> (String, Vec<rbs::Value>) {
                    use rbatis_codegen::ops::*;
                    let mut sql = String::with_capacity(1063usize);
                    let mut args = Vec::with_capacity(1usize);
                    sql.push_str(
                        &::alloc::__export::must_use({
                            let res = ::alloc::fmt::format(
                                format_args!(
                                    "delete from {0} where {1} in (",
                                    &{ &arg["table_name"] }.string(),
                                    &{ &arg["column"] }.string(),
                                ),
                            );
                            res
                        }),
                    );
                    sql.push_str(
                        &{
                            let mut sql = String::with_capacity(10usize);
                            for (ref index, item) in { &arg["column_values"] } {
                                args.push(rbs::to_value({ item }).unwrap_or_default());
                                sql.push_str("?,");
                            }
                            sql = sql
                                .trim_start_matches(",")
                                .trim_end_matches(",")
                                .to_string();
                            sql
                        },
                    );
                    sql.push_str(")");
                    return (sql, args);
                }
                let (mut sql, rb_args) = do_py_sql(rbs::Value::Map(rb_arg_map), '?');
                use rbatis::executor::Executor;
                executor.exec(&sql, rb_args).await
            }
            let mut table_name = String::new();
            table_name = "product".to_string();
            pub fn snake_name() -> String {
                "product_entity".to_string()
            }
            if table_name.is_empty() {
                table_name = snake_name();
            }
            delete_in_column(executor, table_name, column, column_values).await
        }
    }
    impl ProductEntity {
        pub async fn delete_by_column_batch<V: serde::Serialize>(
            executor: &dyn ::rbatis::executor::Executor,
            column: &str,
            values: &[V],
            batch_size: u64,
        ) -> std::result::Result<::rbatis::rbdc::db::ExecResult, ::rbatis::rbdc::Error> {
            let mut rows_affected = 0;
            let ranges = ::rbatis::plugin::Page::<
                (),
            >::make_ranges(values.len() as u64, batch_size);
            for (offset, limit) in ranges {
                rows_affected
                    += <ProductEntity>::delete_in_column(
                            executor,
                            column,
                            &values[offset as usize..limit as usize],
                        )
                        .await?
                        .rows_affected;
            }
            Ok(::rbatis::rbdc::db::ExecResult {
                rows_affected: rows_affected,
                last_insert_id: rbs::Value::Null,
            })
        }
    }
}
