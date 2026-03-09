// Importa todo lo necesario del prelude de Anchor
use anchor_lang::prelude::*;

// Identificador único del programa en Solana
declare_id!("CPydZL5gyh3Tk5Zj28tnJrF1XbVefPgzpQu5VKjhEDRF");

#[program]
pub mod tienda_bolsas {
    use super::*;

    // Función para crear una nueva tienda
    pub fn crear_tienda(context: Context<NuevaTienda>, nombre: String) -> Result<()> {

        // Guardamos el public key del dueño
        let owner_id = context.accounts.owner.key();
        msg!("Owner id: {}", owner_id);

        // Inicializamos la lista de bolsas vacía
        let productos: Vec<Bolsa> = Vec::new();

        // Creamos la cuenta Tienda con sus datos iniciales
        context.accounts.tienda.set_inner(Tienda {
            owner: owner_id,
            nombre,
            productos,
        });

        Ok(())
    }

    // Función para agregar una bolsa a la tienda
    pub fn agregar_producto(context: Context<NuevoProducto>, nombre: String, precio: u16) -> Result<()> {
        // Validamos que el firmante sea el dueño
        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        // Creamos la nueva bolsa
        let bolsa = Bolsa {
            nombre,
            precio,
            disponible: true,
        };

        // La agregamos al vector de productos
        context.accounts.tienda.productos.push(bolsa);

        Ok(())
    }

    // Función para eliminar una bolsa por nombre
    pub fn eliminar_producto(context: Context<NuevoProducto>, nombre: String) -> Result<()> {
        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let productos = &mut context.accounts.tienda.productos;

        // Buscamos la bolsa por nombre y la eliminamos
        for i in 0..productos.len() {
            if productos[i].nombre == nombre {
                productos.remove(i);
                msg!("Producto {} eliminado!", nombre);
                return Ok(());
            }
        }

        // Si no existe, devolvemos error
        Err(Errores::ProductoNoExiste.into())
    }

    // Función para ver todas las bolsas registradas
    pub fn ver_productos(context: Context<NuevoProducto>) -> Result<()> {
        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        // Mostramos la lista completa en logs
        msg!("Lista de productos: {:#?}", context.accounts.tienda.productos);
        Ok(())
    }

    // Función para alternar la disponibilidad de una bolsa
    pub fn alternar_disponibilidad(context: Context<NuevoProducto>, nombre: String) -> Result<()> {
        require!(
            context.accounts.tienda.owner == context.accounts.owner.key(),
            Errores::NoEresElOwner
        );

        let productos = &mut context.accounts.tienda.productos;

        // Buscamos la bolsa y cambiamos su estado disponible
        for i in 0..productos.len() {

            let estado = productos[i].disponible;

            if productos[i].nombre == nombre {

                let nuevo_estado = !estado;
                productos[i].disponible = nuevo_estado;

                msg!("El producto {} ahora tiene disponibilidad: {}", nombre, nuevo_estado);

                return Ok(());
            }
        }

        Err(Errores::ProductoNoExiste.into())
    }

    // Función para contar cuántas bolsas hay registradas
    pub fn total_productos(context: Context<NuevoProducto>) -> Result<()> {

        let total = context.accounts.tienda.productos.len();

        msg!("La tienda tiene {} productos registrados", total);

        Ok(())
    }
}

// Definición de errores personalizados
#[error_code]
pub enum Errores {
    #[msg("Error, no eres el propietario de la tienda")]
    NoEresElOwner,

    #[msg("El producto no existe")]
    ProductoNoExiste,
}

// Definición de la cuenta Tienda
#[account]
#[derive(InitSpace)]
pub struct Tienda {

    // Public key del dueño
    owner: Pubkey,

    // Nombre de la tienda (máx 60 caracteres)
    #[max_len(60)]
    nombre: String,

    // Lista de bolsas (máx 10 elementos)
    #[max_len(10)]
    productos: Vec<Bolsa>,
}

// Definición de la estructura Bolsa
#[derive(AnchorSerialize, AnchorDeserialize, Clone, InitSpace, PartialEq, Debug)]
pub struct Bolsa {

    // Nombre de la bolsa (máx 60 caracteres)
    #[max_len(60)]
    nombre: String,

    // Precio de la bolsa
    precio: u16,

    // Estado de disponibilidad (true/false)
    disponible: bool,
}

// Contexto para crear una nueva tienda
#[derive(Accounts)]
pub struct NuevaTienda<'info> {

    // El dueño debe ser firmante
    #[account(mut)]
    pub owner: Signer<'info>,

    // Inicializamos la cuenta Tienda con seeds y espacio
    #[account(
        init,
        payer = owner,
        space = Tienda::INIT_SPACE + 8,
        seeds = [b"tienda", owner.key().as_ref()],
        bump
    )]
    pub tienda: Account<'info, Tienda>,

    // Programa del sistema
    pub system_program: Program<'info, System>,
}

// Contexto para agregar/eliminar productos
#[derive(Accounts)]
pub struct NuevoProducto<'info> {

    // El dueño debe firmar
    pub owner: Signer<'info>,

    // La tienda debe ser mutable
    #[account(mut)]
    pub tienda: Account<'info, Tienda>,
}
