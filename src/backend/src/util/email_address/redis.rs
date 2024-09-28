use super::EmailAddress;
use redis::{RedisWrite, ToRedisArgs};

impl ToRedisArgs for EmailAddress {
    fn write_redis_args<W: ?Sized + RedisWrite>(&self, out: &mut W) {
        ToRedisArgs::write_redis_args(&self.0, out);
    }
}
