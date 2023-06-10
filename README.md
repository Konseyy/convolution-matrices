Programma izveidota valodā Rust Repozitorija pieejama https://github.com/Konseyy/convolution-matrices

Atverot .exe failu, jāievada relatīvs path uz bildi, ko vēlas mainīt, repozitorijā pievienoju arī testēšanas nolūkos bildi test.png, attiecīgi atverot programmu, pieņemot ka bilde atrodas vienā folderī ar programmu tās vietu var norādīt vienkārši kā "test.gif"

Otrais prasītais ievades parametrs nosaka vai tiks lietoti sharpen vai blur efekti ("t" priekš sharpen, "f" priekš blur)

Pēc programmas izpildes tiek izveidots jauns folderis "images", kur tiek saglabāta bilde "comparison.png", kuras kreisajā pusē ir redzams oriģinālais fails, vidējā redzams efekts ar pielietojot 3x3 konvolūcijas matricu, un labajā pusē redzams efekts pielietojot 5x5 konvolūcijas matricu
