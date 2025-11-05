# 🎯 Bitcoin Puzzle Hunt - Pitch para Jurados (3 minutos)

## Slide 1: Gancho (15seg)

**"Quem aqui já participou de uma caça ao tesouro?"**

Agora imagine: uma caça ao tesouro onde:
- ❌ Você NÃO precisa confiar no organizador
- ❌ NÃO existe possibilidade de trapaça
- ✅ As regras são MATEMATICAMENTE garantidas
- ✅ O primeiro que achar AUTOMATICAMENTE leva o prêmio

**Isso é Bitcoin Puzzle Hunt.**

---

## Slide 2: O Problema (30seg)

**Contratos inteligentes são:**
1. Difíceis de entender
2. Intimidadores para iniciantes
3. Vistos apenas como "finanças descentralizadas"

**Resultado:**
- Baixa adoção
- Medo de usar
- Casos de uso limitados

---

## Slide 3: Nossa Solução (45seg)

**Um JOGO on-chain que qualquer um entende:**

```
1. Organizador escolhe uma senha (ex: "satoshi")
2. Bloqueia Bitcoin com SHA256 dessa senha
3. Publica hints progressivos
4. Primeiro a descobrir a senha → ganha TUDO automaticamente
```

**Por que isso importa?**
- 🎓 **Educacional**: Ensina criptografia de forma divertida
- 🔓 **Acessível**: Qualquer um pode participar
- 🔒 **Trustless**: Matemática garante as regras
- 🌍 **Global**: Sem fronteiras ou permissões

---

## Slide 4: Demo ao Vivo (60seg)

**[MOSTRAR NA TELA]**

```bash
# 1. Criar puzzle com prêmio de 0.1 BTC
$ cargo run --bin create-puzzle -- "hello" 0.1

🎯 Puzzle criado!
📍 Endereço: tex1q...
🔐 Hash: 0x2cf24dba...
💰 Prêmio: 0.1 L-BTC

# 2. Hints para o público
Hint 1: É uma saudação comum em inglês
Hint 2: Tem 5 letras
Hint 3: Começa com "h"

# 3. Alguém resolve
$ cargo run --bin solve-puzzle -- puzzle.json "hello" <endereco>

🎉 SUCESSO! Prêmio enviado!
TXID: abc123...
```

**[MOSTRAR no Block Explorer a transação confirmada]**

---

## Slide 5: Tech Stack (20seg)

- **Simplicity**: Linguagem de contratos formalmente verificável
- **Liquid Network**: Sidechain do Bitcoin (Blockstream)
- **Taproot**: Última upgrade do Bitcoin para privacy
- **Rust**: Performance + segurança

**Único projeto do hackathon usando Simplicity!**

---

## Slide 6: Casos de Uso Além do Jogo (20seg)

1. **Herança Digital**
   - Família precisa juntar partes do secret
   - Cada herdeiro tem uma parte

2. **Educational CTF**
   - Ensinar segurança através de puzzles
   - Recompensas reais por aprender

3. **Marketing Campaigns**
   - Empresas criam puzzles virais
   - Engajamento orgânico

4. **Proof of Knowledge**
   - Provar que você sabe algo sem revelar
   - Academic credentials

---

## Slide 7: Tração / Próximos Passos (20seg)

**Já temos:**
- ✅ Contrato funcional na testnet
- ✅ CLI tools completas
- ✅ Documentação

**Próximos 3 meses:**
- 🌐 Web interface (React + Web3)
- ⏰ Time-locked puzzles
- 🏆 Leaderboard e NFTs
- 💰 Fundraise via puzzles

**Meta:** Lançar 1º torneio global em Q2 2025

---

## Slide 8: Diferencial Competitivo (20seg)

**Por que somos únicos:**

| Feature | Nós | Concorrentes |
|---------|-----|--------------|
| Verificação On-chain | ✅ | ❌ (usam oráculos) |
| Trustless | ✅ | ❌ (precisam de backend) |
| Formally Verified | ✅ | ❌ |
| Open Source | ✅ | ❌ |
| Bitcoin Native | ✅ | ❌ (Ethereum) |

---

## Slide 9: Call to Action (10seg)

**"Vamos testar AGORA?"**

1. Acesse: `github.com/seu-repo/puzzle-hunt`
2. Secret está escondido neste QR code 👇
3. Primeiro a descobrir ganha 0.01 BTC!

[QR CODE com hints]

**Perguntas?**

---

## 🎬 Script para Praticar

### Abertura (com energia!)
"Oi jurados! Vocês já viram aqueles quadros de senha do Tinder? Onde a pessoa coloca um desafio e quem resolver ganha o match? Pois é, fizemos isso... mas com BITCOIN! E sem precisar confiar em ninguém."

### Meio (explicativo)
"A mágica está aqui [apontar para código]: Este contrato Simplicity verifica matematicamente se você sabe a senha. Não tem servidor, não tem admin, não tem como trapacear. É pura matemática."

### Fechamento (impacto)
"Não estamos apenas criando um jogo. Estamos mostrando que blockchain pode ser divertido, educacional, e acessível. Imagina escolas usando isso para ensinar criptografia. Imagina empresas criando campanhas virais on-chain. Imagina famílias protegendo heranças digitais de forma gamificada."

**"O futuro de contratos inteligentes não é apenas DeFi. É engajamento humano real."**

---

## 💡 Dicas de Apresentação

1. **Comece com demo ao vivo** - Mostre funcionando primeiro
2. **Use analogias** - Compare com coisas conhecidas
3. **Energia!** - Este é um projeto empolgante, mostre isso
4. **Menos texto** - Slides visuais, você fala o resto
5. **Prepare para perguntas:**
   - "Por que não Ethereum?" → Bitcoin é mais seguro, Simplicity é formalmente verificável
   - "Como lucram?" → Taxas de plataforma, puzzles sponsorados
   - "Já tem usuários?" → Testnet, lançamento em breve

---

## 🎯 Objetivos da Apresentação

- [ ] Jurados entendem o conceito em 30 segundos
- [ ] Jurados veem funcionando (demo)
- [ ] Jurados entendem o diferencial técnico
- [ ] Jurados veem os casos de uso além do jogo
- [ ] Jurados querem testar/investir

---

**Boa sorte! 🚀**
