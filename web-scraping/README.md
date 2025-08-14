# Rust Web Scraping Projesi

Bu proje, Rust programlama dili kullanılarak web scraping işlemleri gerçekleştirir. Belirtilen e-ticaret sitesinden ürün verileri çekilir, bu veriler CSV dosyasına kaydedilir ve MongoDB veritabanına aktarılır.
Scrapping yapılan site linki : https://www.scrapingcourse.com/ecommerce/

## Kullanılan Kütüphaneler

- **reqwest**: HTTP istekleri yapmak için kullanılır. (blocking özelliği ile senkron istekler yapılır.)
- **scraper**: HTML belgelerini parse etmek ve veri çekmek için kullanılır.
- **csv**: Verileri CSV dosyalarına yazmak ve okumak için kullanılır.
- **mongodb**: MongoDB veritabanı ile etkileşim kurmak için kullanılır.
- **tokio**: Asenkron işlemler için runtime sağlar.
- **serde**: Veri serileştirme ve deserileştirme işlemleri için kullanılır.

## Proje Yapısı

Proje, aşağıdaki adımları içerir:

1. **Web Scraping**: Belirtilen web sitesinden ürün bilgileri (isim, fiyat vb.) çekilir.
2. **CSV Kaydetme**: Çekilen veriler bir CSV dosyasına kaydedilir.
3. **MongoDB'ye Aktarma**: Veriler, Docker üzerinde çalışan bir MongoDB veritabanına aktarılır.

## Kurulum

### Gereksinimler

- Rust (https://www.rust-lang.org/tools/install)
- Docker (https://docs.docker.com/get-docker/)

### MongoDB Container'ı Başlatma

MongoDB container'ını başlatmak için aşağıdaki komutu kullanın:

```bash
docker run -d --name my-mongodb -p 27017:27017 mongo:latest
