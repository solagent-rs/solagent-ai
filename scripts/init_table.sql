CREATE TABLE splash (
    id SERIAL PRIMARY KEY,
    country_full_name VARCHAR(100) NOT NULL,
    country_code VARCHAR(10) NOT NULL,
    country_name_en VARCHAR(100) NOT NULL,
    population BIGINT NOT NULL,
    area FLOAT NOT NULL
);

INSERT INTO splash (country_full_name, country_code, country_name_en, population, area) VALUES
('中华人民共和国', 'CN', 'China', 1409517397, 9596961),
('印度共和国', 'IN', 'India', 1380004385, 3287263),
('美国', 'US', 'United States', 331002651, 9372610),
('印度尼西亚共和国', 'ID', 'Indonesia', 273523615, 1904569),
('巴基斯坦伊斯兰共和国', 'PK', 'Pakistan', 220892340, 881913),
('巴西联邦共和国', 'BR', 'Brazil', 212559417, 8515767),
('尼日利亚联邦共和国', 'NG', 'Nigeria', 206139589, 923768),
('孟加拉人民共和国', 'BD', 'Bangladesh', 166303498, 147570),
('俄罗斯联邦', 'RU', 'Russia', 145912025, 17098242),
('墨西哥合众国', 'MX', 'Mexico', 128932753, 1964375),
('日本国', 'JP', 'Japan', 126476461, 377975),
('埃塞俄比亚联邦民主共和国', 'ET', 'Ethiopia', 114114144, 1104300),
('菲律宾共和国', 'PH', 'Philippines', 109581078, 300000),
('埃及阿拉伯共和国', 'EG', 'Egypt', 102334155, 1002450),
('越南社会主义共和国', 'VN', 'Vietnam', 97338579, 331212),
('刚果民主共和国', 'CD', 'Democratic Republic of the Congo', 89561403, 2344858),
('土耳其共和国', 'TR', 'Turkey', 84339067, 783356),
('伊朗伊斯兰共和国', 'IR', 'Iran', 83992949, 1648195),
('泰国王国', 'TH', 'Thailand', 69425401, 513120),
('英国大不列颠及北爱尔兰联合王国', 'GB', 'United Kingdom', 67886011, 243610);