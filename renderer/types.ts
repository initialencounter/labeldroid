export interface ImageInfo {
  name: string;
  has_annotation: boolean;
}

export interface Shape {
  label: string;
  points: number[][];
  group_id: number | null;
  shape_type: string;
  flags: Record<string, any>;
}

export interface LabelmeData {
  version: string;
  flags: Record<string, any>;
  shapes: Shape[];
  imagePath: string;
  imageData: string | null;
  imageHeight: number;
  imageWidth: number;
}
