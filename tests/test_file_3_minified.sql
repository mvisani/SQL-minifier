CREATE TABLE container_horizontal_rules(id UUID PRIMARY KEY REFERENCES describables(id)ON DELETE CASCADE,item_type_id UUID REFERENCES item_categories(id)ON DELETE CASCADE,other_item_type_id UUID REFERENCES item_categories(id)ON DELETE CASCADE,minimum_temperature FLOAT DEFAULT NULL,maximum_temperature FLOAT DEFAULT NULL,minimum_humidity FLOAT DEFAULT NULL,maximum_humidity FLOAT DEFAULT NULL,minimum_pressure FLOAT DEFAULT NULL,maximum_pressure FLOAT DEFAULT NULL,CHECK(minimum_temperature IS NULL OR maximum_temperature IS NULL OR minimum_temperature<=maximum_temperature),CHECK(minimum_humidity IS NULL OR maximum_humidity IS NULL OR minimum_humidity<=maximum_humidity),CHECK(minimum_pressure IS NULL OR maximum_pressure IS NULL OR minimum_pressure<=maximum_pressure))