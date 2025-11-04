 ## Kira Teminat Kilidi (2-of-2 Multisig Escrow)
 
 Soroban akıllı sözleşmesi ile kiracı ve kiralayanın depozito fonlarını, her iki tarafın da onayı (2-of-2) olmadan serbest bırakmayan merkeziyetsiz teminat sistemi.
 
 - Kiracı, sözleşmeyi başlatır ve token cinsinden depozito tutarını belirtir.
 - Kiracı `fund` ile tutarı kontrata taşır.
 - Serbest bırakma (`release`) veya iade (`refund`) işlemleri için hem kiracı hem kiralayan aynı çağrıda yetkilendirme verir.
 
 Soroban başlangıç örnekleri ve çalışma akışları için: [soroban-examples](https://github.com/stellar/soroban-examples).
 
 ### Proje Yapısı
 
 - `contracts/kira_teminat`: Akıllı sözleşme kaynak kodu
 - `Cargo.toml`: Workspace tanımı
 
 ### Önkoşullar
 
 - Rust toolchain
 - Stellar CLI (Soroban özellikleri ile)
 - Derleme için `soroban-sdk` ve `soroban-token-sdk` bağımlılıkları
 
 ### Derleme
 
 Windows komut satırı:
 
 ```bat
 cd contracts\kira_teminat
 stellar contract build
 ```
 
 Çıktı wasm dosyası: `target/wasm32v1-none/release/kira_teminat.wasm`
 
 ### Test (isteğe bağlı)
 
 Yerel testler örnek token kurulumuna bağlıdır. Testler `#[ignore]` ile işaretlidir. Çalıştırmak için `--ignored` bayrağını kullanın:
 
 ```bat
 cd contracts\kira_teminat
 cargo test -- --ignored
 ```
 
 ### Kimlik Oluşturma (Testnet)
 
 ```bat
 stellar keys generate --global alice --network testnet --fund
 stellar keys generate --global bob   --network testnet --fund
 ```
 
 - `alice`: kiracı
 - `bob`: kiralayan
 
 Alicenin public key'ini görmek için:
 
 ```bat
 stellar keys address alice
 ```
 
 Bu adımlar ve genel akış için referans: [soroban-examples README](https://github.com/stellar/soroban-examples).
 
 ### Token ve Sözleşme Dağıtımı (Testnet)
 
 1) Token sözleşmesini dağıtın (örnek: mevcut bir token kullanabilir veya yeni bir token dağıtabilirsiniz). Stellar CLI token dağıtım ve mint adımlarınızı kendi token süreçlerinize göre uygulayın.
 
 2) Escrow sözleşmesini dağıtın:
 
 ```bat
 cd contracts\kira_teminat
 stellar contract deploy ^
   --wasm target\wasm32v1-none\release\kira_teminat.wasm ^
   --source alice ^
   --network testnet ^
   --alias kira_teminat
 ```
 
 Çıktıdaki sözleşme `ID` veya `--alias kira_teminat` ile çağırabilirsiniz.
 
 ### Kullanım
 
 Aşağıdaki örneklerde `TOKEN_ID` olarak ya dağıttığınız token sözleşme adresini ya da testnet'teki uygun token sözleşme adresini girin. `AMOUNT` depozito tutarıdır (i128).
 
 1) Başlat (init):
 
 ```bat
 stellar contract invoke ^
   --id kira_teminat ^
   --source alice ^
   --network testnet ^
   -- ^
   init ^
   --tenant $(stellar keys address alice) ^
   --landlord $(stellar keys address bob) ^
   --token TOKEN_ID ^
   --amount AMOUNT
 ```
 
 2) Fonla (fund): Kiracının cüzdanından kontrata `AMOUNT` transfer eder.
 
 ```bat
 stellar contract invoke ^
   --id kira_teminat ^
   --source alice ^
   --network testnet ^
   -- ^
   fund
 ```
 
 3) Serbest bırak (release): Hem kiracı hem kiralayan aynı çağrı için yetki vermelidir. CLI ile çoklu kimlik kullanımı için iki tarafın da onayını içerecek şekilde işlem imzalama akışınızı uygulayın.
 
 ```bat
 stellar contract invoke ^
   --id kira_teminat ^
   --source alice ^
   --network testnet ^
   -- ^
   release ^
   --receiver $(stellar keys address bob)
 ```
 
 Not: 2-of-2 gereksinimi için aynı işlemi `bob` da imzalamalıdır. Tek bir çağrıda iki yetki sunulacak şekilde CLI iş akışınızı yapılandırın.
 
 4) İade (refund): Her iki tarafın onayı ile kiracıya iade eder.
 
 ```bat
 stellar contract invoke ^
   --id kira_teminat ^
   --source alice ^
   --network testnet ^
   -- ^
   refund
 ```
 
 5) Görüntüleme (opsiyonel):
 
 ```bat
 stellar contract invoke --id kira_teminat --network testnet -- get_config
 stellar contract invoke --id kira_teminat --network testnet -- is_funded_view
 ```
 
 ### Güvenlik ve Notlar
 
 - İşlevler `require_auth` ile 2-of-2 onay gerektirir.
 - İhtiyaca göre zaman aşımı, arabulucu gibi ek durum makineleri eklenebilir.
 - Bu proje eğitim amaçlıdır, üretim için denetim gerektirir.
 
 ### Referanslar
 
 - Soroban örnekleri ve CLI akışları: [soroban-examples](https://github.com/stellar/soroban-examples)


