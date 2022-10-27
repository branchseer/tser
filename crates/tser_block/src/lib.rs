use std::convert::Infallible;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum BlockChild {
    Line(String),
    SubBlock(Block),
}

pub trait BlockModifier {
    fn modify_block(self, _: &mut Block);
}

impl BlockModifier for String {
    fn modify_block(self, target: &mut Block) {
        target.children.push(BlockChild::Line(self))
    }
}
impl<'a> BlockModifier for &'a str {
    fn modify_block(self, target: &mut Block) {
        target.children.push(BlockChild::Line(self.to_string()))
    }
}
impl<T: BlockModifier> BlockModifier for Option<T> {
    fn modify_block(self, target: &mut Block) {
        if let Some(unwrapped) = self {
            unwrapped.modify_block(target)
        }
    }
}

struct NewBlock<T>(T);
impl<T: BlockModifier> BlockModifier for NewBlock<T> {
    fn modify_block(self, target: &mut Block) {
        let mut new_block = Block::default();
        self.0.modify_block(&mut new_block);
        target.children.push(BlockChild::SubBlock(new_block))
    }
}

struct Iter<T>(T);
impl<B: BlockModifier, T: IntoIterator<Item = B>> BlockModifier for Iter<T> {
    fn modify_block(self, target: &mut Block) {
        for item in self.0 {
            item.modify_block(target);
        }
    }
}

pub fn block<B: BlockModifier, T: IntoIterator<Item = B>>(iter: T) -> impl BlockModifier {
    NewBlock(Iter(iter))
}
pub fn flatten<B: BlockModifier, T: IntoIterator<Item = B>>(iter: T) -> impl BlockModifier {
    Iter(iter)
}

impl BlockModifier for Block {
    fn modify_block(self, target: &mut Block) {
        target.children.push(BlockChild::SubBlock(self))
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct Block {
    children: Vec<BlockChild>,
}

impl IntoIterator for Block {
    type Item = BlockChild;
    type IntoIter = std::vec::IntoIter<BlockChild>;
    fn into_iter(self) -> Self::IntoIter {
        self.children.into_iter()
    }
}

impl BlockModifier for BlockChild {
    fn modify_block(self, target: &mut Block) {
        target.children.push(self)
    }
}

#[macro_export]
macro_rules! block {
    ($($x:expr),* $(,)?) => {{
        let mut __block = $crate::Block::default();
        $($crate::BlockModifier::modify_block($x, &mut __block);)*
        __block
    }};
}

#[macro_export]
macro_rules! flatten {
    ($($x:expr),* $(,)?) => {
        $crate::flatten($crate::block![$($x),*])
    };
}

const IDENT: &str = "    ";
impl Block {
    fn emit_str<E, F: FnMut(&str) -> Result<(), E>>(
        &self,
        on_str: &mut F,
        level: u64,
    ) -> Result<(), E> {
        for child in &self.children {
            match child {
                BlockChild::Line(line) => {
                    if !line.is_empty() {
                        for _ in 0..level {
                            on_str(IDENT)?;
                        }
                        on_str(line)?;
                    }
                    on_str("\n")?;
                }
                BlockChild::SubBlock(sub_block) => sub_block.emit_str(on_str, level + 1)?,
            }
        }
        Ok(())
    }

    pub fn string(&self) -> String {
        let mut result = String::new();
        let _: Result<(), Infallible> = self.emit_str(
            &mut |s| {
                result.push_str(s);
                Ok(())
            },
            0,
        );
        result
    }
}

#[cfg(test)]
mod tests {
    use super::{block, flatten, Block, BlockChild};

    #[test]
    fn empty() {
        assert_eq!(block![], Block { children: vec![] })
    }

    #[test]
    fn string_types() {
        let b = block!["foo", format!("{}", 1 + 1),];
        assert_eq!(
            b,
            Block {
                children: vec![
                    BlockChild::Line("foo".to_string()),
                    BlockChild::Line("2".to_string()),
                ]
            }
        )
    }

    #[test]
    fn nested() {
        let b = block!["a", block!["foo", "bar"], "b"];
        assert_eq!(
            b,
            Block {
                children: vec![
                    BlockChild::Line("a".to_string()),
                    BlockChild::SubBlock(Block {
                        children: vec![
                            BlockChild::Line("foo".to_string()),
                            BlockChild::Line("bar".to_string()),
                        ]
                    }),
                    BlockChild::Line("b".to_string()),
                ]
            }
        )
    }
    #[test]
    fn block_iter() {
        let b = block!["a", block((0..3).map(|x| (x * 2).to_string())), "b"];
        assert_eq!(
            b,
            Block {
                children: vec![
                    BlockChild::Line("a".to_string()),
                    BlockChild::SubBlock(Block {
                        children: vec![
                            BlockChild::Line("0".to_string()),
                            BlockChild::Line("2".to_string()),
                            BlockChild::Line("4".to_string()),
                        ]
                    }),
                    BlockChild::Line("b".to_string()),
                ]
            }
        )
    }

    #[test]
    fn flatten_iter() {
        let b = block!["a", flatten((0..3).map(|x| (x * 2).to_string())), "b"];
        assert_eq!(
            b,
            Block {
                children: vec![
                    BlockChild::Line("a".to_string()),
                    BlockChild::Line("0".to_string()),
                    BlockChild::Line("2".to_string()),
                    BlockChild::Line("4".to_string()),
                    BlockChild::Line("b".to_string()),
                ]
            }
        )
    }
    #[test]
    fn flatten_macro() {
        let b = block!["a", flatten!["x", "y"], "b"];
        assert_eq!(
            b,
            Block {
                children: vec![
                    BlockChild::Line("a".to_string()),
                    BlockChild::Line("x".to_string()),
                    BlockChild::Line("y".to_string()),
                    BlockChild::Line("b".to_string()),
                ]
            }
        )
    }

    #[test]
    fn block_string() {
        let s = block!["a", block!["b", "", "c", block!["d"]], "e"];
        assert_eq!(s.string(), "a\n    b\n\n    c\n        d\ne\n");
    }
}
