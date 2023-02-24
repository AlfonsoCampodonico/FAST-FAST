import * as fs from 'fs';
import csvParser from 'csv-parser';

interface WeatherData {
  barrio?: string;
  dia?: string;
  direccion_viento_tarde?: string;
  direccion_viento_temprano?: string;
  horas_de_sol?: number;
  humedad_tarde?: number;
  humedad_temprano?: number;
  id?: number;
  llovieron_hamburguesas_hoy?: boolean;
  mm_evaporados_agua?: number;
  mm_lluvia_dia?: number;
  nubosidad_tarde?: number;
  nubosidad_temprano?: number;
  presion_atmosferica_tarde?: number;
  presion_atmosferica_temprano?: number;
  rafaga_viento_max_direccion?: string;
  rafaga_viento_max_velocidad?: number;
  temp_max?: number;
  temp_min?: number;
  temperatura_tarde?: number;
  temperatura_temprano?: number;
  velocidad_viendo_tarde?: number;
  velocidad_viendo_temprano?: number;
}

function parseCsvToJSON(inputFilePath: string, outputDirPath: string): void {
  fs.createReadStream(inputFilePath)
    .pipe(csvParser())
    .on('data', (data: WeatherData) => {
      const json = JSON.stringify(data);
      const filename = `${data.barrio}_${data.dia}.json`;
      fs.writeFileSync(`${outputDirPath}/${filename}`, json);
    })
    .on('end', () => {
      console.log('Parsing CSV to JSON completed successfully');
    });
}

parseCsvToJSON('../data/data.csv', '../data/typescript');