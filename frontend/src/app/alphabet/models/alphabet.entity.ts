import { UserDateEntity } from './user-date.entity';

export interface AlphabetEntity {
  id: string;
  title: string;
  user_dates: UserDateEntity[];
}
