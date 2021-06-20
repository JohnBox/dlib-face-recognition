extern crate serde;

use std::fmt;
use std::ops::Deref;
use std::slice;

use serde::{de::{SeqAccess, Visitor}, Deserialize, Deserializer};

/// A wrapper around a `matrix<double,0,1>>`, an encoding.
#[derive(Clone)]
pub struct FaceEncoding {
    inner: FaceEncodingInner,
}

cpp_class!(unsafe struct FaceEncodingInner as "dlib::matrix<double,0,1>");

impl FaceEncoding {
    pub fn new() -> Self {
        Self::new_from_scalar(0.)
    }


    pub fn new_from_scalar(scalar: f64) -> Self {
        let inner = unsafe {
            cpp!([scalar as "double"] -> FaceEncodingInner as "dlib::matrix<double,0,1>" {
                auto inner = dlib::matrix<double,0,1>(128);
                for (auto i = 0; i < 128; i++) {
                    inner(i) = scalar;
                }

                return inner;
            })
        };

        Self { inner }
    }

    /// Calculate the euclidean distance between two encodings.
    ///
    /// This value can be compared to a constant to determine if the faces are the same or not.
    /// A good value for this is `0.6`.
    pub fn distance(&self, other: &Self) -> f64 {
        unsafe {
            cpp!([self as "const dlib::matrix<double,0,1>*", other as "const dlib::matrix<double,0,1>*"] -> f64 as "double" {
                return dlib::length(*self - *other);
            })
        }
    }
}

impl From<Vec<f64>> for FaceEncoding {
    fn from(encoding: Vec<f64>) -> Self {
        let inner = unsafe {
            cpp!([encoding as "std::vector<double>"] -> FaceEncodingInner as "dlib::matrix<double,0,1>" {
                auto inner = dlib::matrix<double,0,1>(128);
                for (auto i = 0; i < 128; i++) {
                    inner(i) = encoding[i];
                }

                return inner;
            })
        };

        Self { inner }
    }
}

impl Deref for FaceEncoding {
    type Target = [f64];

    fn deref(&self) -> &Self::Target {
        let matrix = &self.inner;

        let len = unsafe {
            cpp!([matrix as "const dlib::matrix<double,0,1>*"] -> usize as "size_t" {
                return matrix->size();
            })
        };

        if len == 0 {
            &[]
        } else {
            unsafe {
                let pointer = cpp!([matrix as "dlib::matrix<double,0,1>*"] -> *const f64 as "double*" {
                    return &(*matrix)(0);
                });

                slice::from_raw_parts(pointer, len)
            }
        }
    }
}

impl fmt::Debug for FaceEncoding {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.deref().fmt(fmt)
    }
}

impl PartialEq for FaceEncoding {
    fn eq(&self, other: &Self) -> bool {
        self.deref().eq(other.deref())
    }
}


impl<'de> Deserialize<'de> for FaceEncoding {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as Deserializer<'de>>::Error>
        where D: Deserializer<'de> {
        struct FaceVisitor;

        impl<'de> Visitor<'de> for FaceVisitor {
            type Value = FaceEncoding;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("json array of 128 numbers")
            }

            fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, <A as SeqAccess<'de>>::Error> where
                A: SeqAccess<'de>, {
                let mut v = Vec::with_capacity(128);

                while let Ok(res) = seq.next_element::<f64>() {
                    if let Some(value) = res {
                        v.push(value)
                    } else {
                        break;
                    }
                }

                Ok(FaceEncoding::from(v))
            }
        }

        deserializer.deserialize_seq(FaceVisitor)
    }
}

#[test]
fn encoding_test() {
    let encoding_a = FaceEncoding::new_from_scalar(0.0);
    let encoding_b = FaceEncoding::new_from_scalar(1.0);

    assert_eq!(encoding_a, encoding_a);
    assert_ne!(encoding_a, encoding_b);

    assert_eq!(encoding_a.distance(&encoding_b), 128.0_f64.sqrt());
}

#[test]
fn deserialize_test() {
    let raw_data = r#"[-0.09236698597669601, 0.10809268802404404, 0.016155723482370377, -0.09244463592767715, -0.1378307193517685, -0.018521230667829514, -0.06456020474433899, -0.08599989861249924, 0.12253346294164658, -0.11098232865333557, 0.19219070672988892, -0.016411494463682175, -0.23379285633563995, 0.0063555920496582985, -0.022636212408542633, 0.16411910951137543, -0.21262815594673157, -0.16641688346862793, -0.06071363762021065, -0.08906292915344238, 0.06471101939678192, 0.021275371313095093, -0.05314533784985542, 0.11202120780944824, -0.1941663920879364, -0.19885076582431793, -0.07450439780950546, -0.09485595673322678, 0.012608764693140984, -0.07697443664073944, 0.09085613489151001, 0.03955479711294174, -0.06635264307260513, 0.05987972021102905, -0.011758342385292053, 0.0392780601978302, 0.004392137750983238, -0.07322853058576584, 0.22088763117790222, 0.0635751485824585, -0.17913684248924255, 0.09524627029895782, -0.029063982889056206, 0.3490733802318573, 0.15651841461658478, 0.03952944278717041, 0.070914626121521, -0.03307904303073883, 0.16237322986125946, -0.28134116530418396, 0.0974094495177269, 0.14325302839279175, 0.15776416659355164, 0.056259747594594955, 0.07538793981075287, -0.13824832439422607, 0.07060879468917847, 0.18239754438400269, -0.2484445422887802, 0.09679818153381348, 0.08076503127813339, -0.04744665324687958, 0.07070011645555496, -0.052807506173849106, 0.25569602847099304, 0.12330153584480286, -0.15479087829589844, -0.05431227385997772, 0.11730876564979553, -0.12881428003311157, -0.17170485854148865, 0.04868616536259651, -0.11507799476385117, -0.15311335027217865, -0.28867870569229126, 0.060020461678504944, 0.3650636374950409, 0.21530143916606903, -0.23827797174453735, 0.0008274335414171219, -0.0004169815219938755, 0.02680990658700466, 0.0890711173415184, 0.07060959190130234, -0.14225882291793823, -0.061759717762470245, -0.12494666874408722, -0.013875722885131836, 0.21187135577201843, 0.009310753084719181, -0.06310561299324036, 0.2986067831516266, 0.10081198066473007, -0.05664318427443504, 0.028835592791438103, 0.04919514060020447, -0.08338546007871628, -0.05737367272377014, -0.024793101474642754, -0.04573812335729599, 0.07191599160432816, -0.04869179427623749, 0.0029402468353509903, 0.08332090824842453, -0.2008308619260788, 0.2442949116230011, -0.06054406613111496, 0.012509076856076717, -0.09819798171520233, 0.07338141649961472, -0.09603449702262878, 0.040870800614356995, 0.16943377256393433, -0.24514085054397583, 0.14059224724769592, 0.15790580213069916, 0.03754998371005058, 0.12035967409610748, 0.06971008330583572, 0.03664987534284592, -0.06381170451641083, -0.04256236553192139, -0.08607235550880432, -0.13304680585861206, 0.10934300720691681, -0.027161311358213425, 0.1561008095741272, 0.09779120981693268]"#;

    let encoding: FaceEncoding = serde_json::from_str(raw_data).unwrap();
    assert_eq!(encoding.len(), 128);
}