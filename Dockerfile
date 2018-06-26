# escape=`

FROM microsoft/dotnet-framework:4.7.2-sdk

SHELL ["powershell", "-Command", "$ErrorActionPreference = 'Stop'; $ProgressPreference = 'SilentlyContinue';"]

WORKDIR C:\install

RUN Invoke-WebRequest -OutFile vs_buildtools.exe 'https://aka.ms/vs/15/release/vs_buildtools.exe'; `
    $expected = 'EA5108C2694B297E0ABDBA52666DE19A70E71FF42249A37524E9448E4BD30CE9'; `
    $actual = (Get-FileHash vs_buildtools.exe -Algorithm sha256).Hash; `
    if ($actual -ne $expected) { `
        Write-Host "CHECKSUM VERIFICATION FAILED: $actual instead of $expected"; `
        exit 1; `
    };

#vs_buildtools.exe --wait --quiet --norestart --nocache --installPath C:\BuildTools --add Microsoft.VisualStudio.Workload.VCTools

# Copy our Install script.
COPY Install.cmd C:\TEMP\

# Download collect.exe in case of an install failure.
ADD https://aka.ms/vscollect.exe C:\TEMP\collect.exe

#START /B /WAIT vs_buildtools.exe --quiet --wait --norestart --nocache --installPath C:\BuildTools --add Microsoft.VisualStudio.Workload.VCTools --add Microsoft.VisualStudio.Component.Windows10SDK.17134
# vs_buildtools.exe --quiet --wait --norestart --nocache --installPath C:\BuildTools --add Microsoft.VisualStudio.Workload.VCTools --add Microsoft.VisualStudio.Component.Windows10SDK.17134
RUN .\vs_buildtools.exe --quiet --wait --norestart --nocache `
    --installPath C:\BuildTools `
    --add Microsoft.VisualStudio.Workload.VCTools `
    --add Microsoft.VisualStudio.Component.Windows10SDK.17134 `
    | Out-Null; `
    Remove-Item -Force vs_buildtools.exe;

ENV RUST_VERSION=1.26.2
RUN [Net.ServicePointManager]::SecurityProtocol = [Net.SecurityProtocolType]::Tls12; `
    Invoke-WebRequest -OutFile rustup-init.exe 'https://static.rust-lang.org/rustup/archive/1.11.0/x86_64-pc-windows-msvc/rustup-init.exe'; `
    $expected = '7F14F767FB547E7B7548341892D0849C9D77EAADA1F5E199B5E8C51F54CF06C0'; `
    $actual = (Get-FileHash rustup-init.exe -Algorithm sha256).Hash; `
    if ($actual -ne $expected) { `
        Write-Host "CHECKSUM VERIFICATION FAILED: $actual instead of $expected"; `
        exit 1; `
    }; `
    .\rustup-init.exe -y --default-toolchain $env:RUST_VERSION;`
    Remove-Item -Force rustup-init.exe;

WORKDIR C:\build

COPY . .

RUN &cargo build