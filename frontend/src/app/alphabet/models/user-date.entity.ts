import { DateIdeaEntity } from '../../date-idea/models/date-idea.entity';

export interface UserDateEntity {
  id: string;
  letter: string;
  completed: boolean;
  date_idea: DateIdeaEntity;
}
