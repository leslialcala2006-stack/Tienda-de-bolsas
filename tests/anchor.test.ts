describe("Tienda de Bolsas", () => {
  it("crear tienda y agregar producto", async () => {
    const ownerKp = new web3.Keypair();

    const [tiendaPda] = web3.PublicKey.findProgramAddressSync(
      [Buffer.from("tienda"), ownerKp.publicKey.toBuffer()],
      pg.program.programId
    );

    const txHash = await pg.program.methods
      .crearTienda("Mi Tienda de Bolsas")
      .accounts({
        tienda: tiendaPda,
        owner: ownerKp.publicKey,
        systemProgram: web3.SystemProgram.programId,
      })
      .signers([ownerKp])
      .rpc();

    console.log(`Transacción: ${txHash}`);

    await pg.connection.confirmTransaction(txHash);

    await pg.program.methods
      .agregarProducto("Bolsa Negra", 120)
      .accounts({
        tienda: tiendaPda,
        owner: ownerKp.publicKey,
      })
      .signers([ownerKp])
      .rpc();

    const tienda = await pg.program.account.tienda.fetch(tiendaPda);
    console.log("Productos registrados:", tienda.productos);

    assert.equal(tienda.productos.length, 1);
    assert.equal(tienda.productos[0].nombre, "Bolsa Negra");
  });
});
