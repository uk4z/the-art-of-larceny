use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct BoardUI;

#[derive(Component, Debug)]
pub struct Helper;

#[derive(Component, Debug)]
pub struct LockedButton;

#[derive(Component, Debug)]
pub struct UnlockedButton;


#[derive(Component, Debug)]
pub struct LoadingBar;

#[derive(Component, Debug)]
pub struct OpenTarget;

#[derive(Resource, Debug)]
pub struct CurrencyLocked(pub bool);

#[derive(Component, Debug)]
pub struct ItemBoard;

#[derive(Component, Debug)]
pub struct StealthStatus;

#[derive(Component, Debug)]
pub struct IntelMenu;

#[derive(Component, Debug)]
pub struct IntelLabel;

#[derive(Component, Debug)]
pub enum Section {
    Vault,
    Instruction, 
    Item
}

#[derive(Bundle, Debug)]
pub struct IntelBundle{
    pub section: Section,
}

#[derive(Component, Debug)]
pub struct Vault;

#[derive(Component, Debug)]
pub struct Password;

#[derive(Component, Debug)]
pub struct PasswordText;

#[derive(Component, Debug)]
pub struct VaultContent;

#[derive(Component, Debug)]
pub struct Amount;

#[derive(Component, Debug)]
pub struct Currency;


#[derive(Component, Debug)]
pub struct Instruction;

#[derive(Component, Debug)]
pub struct ItemContent;