export interface Photo {
  id: number;
  path: string;
  filename: string;
  file_size: number | null;
  date_taken: string | null;
  width: number | null;
  height: number | null;
  format: string;
}
