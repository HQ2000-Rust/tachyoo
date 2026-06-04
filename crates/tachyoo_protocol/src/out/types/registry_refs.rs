use tokio::io;

use crate::out::{
    Transfer, Writable,
    types::{
        array::Array,
        identifier::Identifier,
        option::unprefixed::Optional,
        var::int::VarInt,
    },
};

//TODO: would enums be better?

pub struct IdOrX<X> {
    //registry ID +1 if X is not given, otherwise 0
    id: VarInt,
    x: Optional<X>,
}

impl<X> IdOrX<X> {
    //TODO: determine valid registry id range!!
    pub fn id(id: i32) -> IdOrX<X> {
        IdOrX {
            id: VarInt::new(id + 1),
            x: Optional::none(),
        }
    }

    pub fn x(x: X) -> IdOrX<X> {
        IdOrX {
            id: VarInt::new(0),
            x: Optional::some(x),
        }
    }
}

#[async_trait::async_trait]
impl<X> Transfer for IdOrX<X>
where
    X: Transfer + Send + Sync,
{
    async fn write_data(&self, writeable: &mut Writable) -> io::Result<()> {
        self.id.write_data(writeable).await?;
        self.x.write_data(writeable).await?;

        Ok(())
    }
}

pub struct IdSet {
    ty: VarInt,
    tag_name: Optional<Identifier>,
    ids: Optional<Array<VarInt>>,
}

impl IdSet {
    pub fn named_id_set(tag_name: Identifier) -> IdSet {
        IdSet {
            ty: VarInt::new(0),
            tag_name: Optional::some(tag_name),
            ids: Optional::none(),
        }
    }

    pub fn inline_ids(ids: impl Iterator<Item = VarInt>) -> IdSet {
        let array = Array::from_iter(ids);
        //TODO: ensure no overflow occurs
        IdSet {
            ty: VarInt::new(array.len() as i32 + 1),
            tag_name: Optional::none(),
            ids: Optional::some(array),
        }
    }
}

#[async_trait::async_trait]
impl Transfer for IdSet {
    async fn write_data(&self, writeable: &mut Writable) -> io::Result<()> {
        self.ty.write_data(writeable).await?;
        self.tag_name.write_data(writeable).await?;
        self.ids.write_data(writeable).await?;

        Ok(())
    }
}
