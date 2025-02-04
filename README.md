Rust Web Scrapping Projesi
Bu proje, Rust programlama dili kullanılarak web scraping işlemleri gerçekleştirir. Aşağıdaki kütüphaneler kullanılarak bir e-ticaret sitesinden ürün verileri çekilir, bu veriler CSV dosyasına kaydedilir ve MongoDB veritabanına aktarılır.

scrapping yapılan site linki : https://www.scrapingcourse.com/ecommerce/

Kullanılan Kütüphaneler
reqwest: HTTP istekleri yapmak için kullanılır. (blocking özelliği ile senkron istekler yapılır.)

scraper: HTML belgelerini parse etmek ve veri çekmek için kullanılır.

csv: Verileri CSV dosyalarına yazmak ve okumak için kullanılır.

mongodb: MongoDB veritabanı ile etkileşim kurmak için kullanılır.

tokio: Asenkron işlemler için runtime sağlar.

serde: Veri serileştirme ve deserileştirme işlemleri için kullanılır.

Proje Yapısı
Proje, aşağıdaki adımları içerir:

Web Scraping: Belirtilen web sitesinden ürün bilgileri (isim, fiyat vb.) çekilir.

CSV Kaydetme: Çekilen veriler bir CSV dosyasına kaydedilir.

MongoDB'ye Aktarma: Veriler, Docker üzerinde çalışan bir MongoDB veritabanına aktarılır.

MongoDB containerı başlatma :
docker run -d --name my-mongodb -p 27017:27017 mongo:latest
