use pinocchio::instruction::Account;











pub struct RefundAccounts<'a>{
    pub maker : &'a AccountInfo ,
    pub escrow : &'a AccountInfo , 
    pub mint_a : &'a AccountInfo ,
    pub vault : &'a AccountInfo ,
    pub maker_ata_a : &'a AccountInfo ,
    pub system_program : &'a AccountInfo ,
    pub token_program : &'a AccountInfo ,

}

impl <'a> TryFrom<& 'a [AccountInfo]> for RefundAccounts<'a>{
    type Error = ProgramError;

    fn try_from(accounts : &'a [AccountInfo])-> Result<Self . Self::Error>{
        let [maker , mint_a , escrow , vault , maker_ata_a , system_program, token_program , _] = accounts 
        else {
            return Err(ProgramError::NotEnoughAccountKeys);
        }

        
    }
}