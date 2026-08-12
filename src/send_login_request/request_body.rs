use serde::Serialize;
use zstd::stream::write::Encoder;

use super::entities::UserCredentials;

#[derive(Serialize)]
struct LoginBody {
    u: String,
    p: String
}

impl LoginBody {
    fn from(credentials: UserCredentials) -> LoginBody {
            LoginBody {
            u: credentials.user,
            p: credentials.password,
        }
    }
}

pub fn build(credentials: UserCredentials) -> Vec<u8> {
    let body = LoginBody::from(credentials);
    let mut encoder = Encoder::new(Vec::new(), 0).unwrap();
    serde_json::to_writer(&mut encoder, &body).unwrap();
    encoder.finish().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_body_compression() {
        assert_eq!(
            build(UserCredentials {
                user: String::from("filip@razek.org"),
                password: String::from("password"),
            }),
            [40, 181, 47, 253, 0, 88, 49, 1, 0, 123, 34, 117, 34, 58, 34, 102, 105, 108, 105, 112, 64, 114, 97, 122, 101, 107, 46, 111, 114, 103, 34, 44, 34, 112, 34, 58, 34, 112, 97, 115, 115, 119, 111, 114, 100, 34, 125]
        );
        assert_eq!(
            build(UserCredentials {
                user: String::from("filip@razek.org"),
                password: String::from("c0Mpl1c4t3D_Pa$$w0Rd"),
            }),
            [40, 181, 47, 253, 0, 88, 145, 1, 0, 123, 34, 117, 34, 58, 34, 102, 105, 108, 105, 112, 64, 114, 97, 122, 101, 107, 46, 111, 114, 103, 34, 44, 34, 112, 34, 58, 34, 99, 48, 77, 112, 108, 49, 99, 52, 116, 51, 68, 95, 80, 97, 36, 36, 119, 48, 82, 100, 34, 125]
        );
        assert_eq!(
            build(UserCredentials {
                user: String::from("filip@razek.org"),
                password: String::from("letmein123"),
            }),
            [40, 181, 47, 253, 0, 88, 65, 1, 0, 123, 34, 117, 34, 58, 34, 102, 105, 108, 105, 112, 64, 114, 97, 122, 101, 107, 46, 111, 114, 103, 34, 44, 34, 112, 34, 58, 34, 108, 101, 116, 109, 101, 105, 110, 49, 50, 51, 34, 125]
        );
    }
}
