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