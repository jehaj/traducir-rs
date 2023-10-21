#! /bin/bash
mkdir raw_data
cd raw_data
wget --output-document=topdatamat.html https://web.archive.org/web/20230202020131/https://topdatamat.dk/ordbog.thc
wget --output-document=klid.html http://www.klid.dk/dansk/ordlister/ordliste.html
wget --output-document=sdu.html https://web.archive.org/web/20220526151932/https://imada.sdu.dk/~chdj/ordbog_en_da.php
