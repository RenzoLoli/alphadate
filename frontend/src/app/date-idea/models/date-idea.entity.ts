import { TagEntity } from "./tag.entity";

export interface DateIdeaEntity {
  id: string;
  idea: string;
  description: string;
  tags: TagEntity[];
}
