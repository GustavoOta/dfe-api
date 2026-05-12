# Doações:
[![Sponsor GustavoOta](https://img.shields.io/badge/sponsor-GustavoOta-%23EA4AAA?style=flat&logo=github)](https://github.com/sponsors/GustavoOta)

# dfe-api
Api Rest DFe 

Emissor de NFe

## Instalação

### Pré-requisitos

- Rust instalado. Você pode instalar o Rust a partir do [site oficial](https://www.rust-lang.org/).

### Passos para Instalação

1. Clone o repositório:
    ```cmd
    git clone https://github.com/GustavoOta/dfe-api.git
    cd dfe-api
    ```

2. Configure as variáveis de ambiente necessárias:
    - `OPENSSL_CONF`: Caminho absoluto para o arquivo `openssl.cnf`.
    - `OPENSSL_MODULES`: Caminho absoluto para o diretório que contém os módulos do OpenSSL.

    - Ao clonar o repositório perceba a pasta dfe, em thirdparty você encontra os arquivos do OPENSSL.

3. Copiar a pasta dfe para o root do executável, pois contém os arquivos necessários para a validação de XML.

4. Compile o projeto usando o script [build-release.bat](http://_vscodecontentref_/1):
    - O script [build-release.bat](http://_vscodecontentref_/2) define a flag `RUSTFLAGS` para compilar o binário com as DLLs estáticas e, em seguida, compila o projeto em modo release.
    - O script também copia o executável gerado para o diretório raiz do projeto.

    Para executar o script, abra o Prompt de Comando e execute:
    ```cmd
    build-release.bat
    ```

    O conteúdo do [build-release.bat](http://_vscodecontentref_/3) é o seguinte:
    ```bat
    @echo off
    @echo Define RUSTFLAGS
    set RUSTFLAGS=-Ctarget-feature=+crt-static

    @echo Build the release
    cargo build --release

    @echo Copy Release to root directory
    copy target\release\*.exe .
    ```

5. Após a compilação, execute o binário gerado:
    ```cmd
    dfe-api.exe
    ```

6. Verifique se as variáveis de ambiente foram configuradas corretamente:
    ```cmd
    echo %OPENSSL_CONF%
    echo %OPENSSL_MODULES%
    ```

Se tudo estiver configurado corretamente, o `dfe-api` estará pronto para uso.

---

## Endpoints

### Distribuição de DFe (Manifestação do Destinatário)

Base path: `/distribuicao`

---

#### POST `/distribuicao/ciencia-operacao`

Registra o evento de **Ciência da Operação** (tpEvento `210210`) para uma NF-e no Ambiente Nacional da SEFAZ.

Indica que o destinatário tem ciência da existência da NF-e, mas ainda não confirmou nem desconheceu a operação.

**Request Body** (`application/json`):

```json
{
  "cert_path": "C:/certificados/empresa.pfx",
  "cert_pass": "senha_do_certificado",
  "cnpj": "10868122000158",
  "ambiente": 2,
  "chave_acesso": "35260402084385000148550010008921991199917393"
}
```

| Campo          | Tipo     | Descrição                                                     |
| -------------- | -------- | ------------------------------------------------------------- |
| `cert_path`    | `string` | Caminho absoluto para o arquivo `.pfx` do certificado digital |
| `cert_pass`    | `string` | Senha do certificado `.pfx`                                   |
| `cnpj`         | `string` | CNPJ do destinatário (apenas dígitos)                         |
| `ambiente`     | `number` | `1` = Produção, `2` = Homologação                             |
| `chave_acesso` | `string` | Chave de acesso da NF-e (44 dígitos)                          |

**Response — Sucesso:**

```json
{
  "error": 0,
  "msg": "Ciência da operação enviada com sucesso",
  "data": {
    "response": {
      "tp_amb": "2",
      "ver_aplic": "AN_1.9.0",
      "c_orgao": "91",
      "c_stat": "135",
      "x_motivo": "Evento registrado e vinculado a NF-e",
      "ch_nfe": "35260402084385000148550010008921991199917393",
      "tp_evento": "210210",
      "n_seq_evento": "1",
      "dh_reg_evento": "2026-04-14T10:16:21-03:00",
      "x_evento": "Ciencia da Operacao",
      "cnpj_dest": "10868122000158",
      "n_prot": "891260002352208"
    },
    "send_xml": "...",
    "receive_xml": "..."
  }
}
```

| Campo          | Descrição                                                                                     |
| -------------- | --------------------------------------------------------------------------------------------- |
| `c_stat`       | `135` = registrado e vinculado à NF-e; `136` = registrado mas NF-e ainda não localizada no AN |
| `n_prot`       | Número do protocolo SEFAZ — recomenda-se persistir para rastreabilidade                       |
| `cnpj_dest`    | CNPJ do destinatário confirmado pelo Ambiente Nacional                                        |
| `n_seq_evento` | Sempre `1` para Ciência da Operação                                                           |
| `send_xml`     | Envelope SOAP enviado à SEFAZ                                                                 |
| `receive_xml`  | Envelope SOAP recebido da SEFAZ                                                               |

**Response — Erro:**

```json
{
  "error": 1,
  "msg": "Erro ao enviar ciência da operação: <detalhe do erro>",
  "data": null
}
```

**Logs gerados** (em `./distribuicao-logs/`):

```
requests/{cnpj}/{ano}/{mes}/ciencia-da-operacao-HH-MM-SS-inf-evento.xml
requests/{cnpj}/{ano}/{mes}/ciencia-da-operacao-HH-MM-SS-envio.xml
responses/{cnpj}/{ano}/{mes}/ciencia-da-operacao-HH-MM-SS-resposta.xml
errors/{cnpj}/{ano}/{mes}/ciencia-da-operacao-HH-MM-SS-erro.txt  ← apenas em caso de falha
```