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
        };

        SignerAccount::check(maker)?;
        ProgramAccount::check(escrow)?;
        MintInterface::check(mint_a)?;
        ProgramAccount::check(vault)?;
        AssociatedTokenAccount::check(vault , escrow , mint_a , token_program)?;

         Ok(Self{
         
            maker,
            escrow, 
            mint_a,
            maker_ata_a,
            vault,
            system_program,
            token_program,
        })

    }
}

pub struct Refund<'a>{
    pub accounts : RefundAccounts<'a>,
}

impl <'a> TryFrom<&'a [AccountInfo]> for Refund<'a>{
    type Error = ProgramError;

    fn try_from(accounts : <&'a [AccountInfo]>)->Result(Self,Self::Error){
        let accounts = RefundAccounts::try_from(accounts)?;

       

        AssociatedTokenAccount::init_if_needed(
            accounts.maker_ata_b,
            accounts.maker,
            accounts.mint_b,
            accounts.taker,
            accounts.token_program,
            accounts.system_program,
        )?;
        Ok(Self{accounts})
    }
}


impl <'a> Refund<'a>{

    pub const DISCRIMINATOR : U8 = &1;

    pub fn  process(&mut Self) -> ProgramResult{
        let data = self.accounts.escrow.try_borrow_data()?;
        let escrow = Escrow::load(&data)?;
        
        let escrow_key = create_program_address(&[b"escrow", self.accounts.maker.key(),&escrow.seed.to_le_bytes(), &escrow.bump],&crate::ID)?;
        if &escrow_key != self.accounts.escrow.key(){
            return Err(ProgramError::InvalidAccountOwner);
        }

        let seed_binding = escrow.seed.to_le_bytes();
        let bump_binding = [escrow.bump];

        let escrow_seeds = [
            Seed::from(b"escrow"),
            Seed::from(self.accounts.maker.key().as_ref()),
            Seed::from(&seed_binding),
            Seed::from(&bump_binding),
        ];

        let signer = Signer::from(&escrow_seeds);

        let amount = TokenAccount::get_amount(self.accounts.vault)

        CloseAccount {
            account : self.accounts.vault ,
            destination :self.accounts.maker,
            authority : self.accounts.escrow,
        }invoke_signed(&[signer.clone()])?;

        Transfer{
            from : self.accounts.vault,
            to : self.accounts.maker_ata_a,
            authority: self.accounts.maker,
            amount:escrow.receive,
        }invoke()?;

        drop(data);

        ProgramAccount::close(self.accounts.escrow , self.accounts.taker)?;

        Ok(())



    } 
}