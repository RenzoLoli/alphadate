import users from "@/../database/users.json";
import { UserAdapter } from "@/adapters/user.adapter";
import { User } from "@/model/user.entity";

export class UserApiService {
  constructor(private db: User[] = []) {
    this.db = users.map((userReq: any) => UserAdapter.requestToEntity(userReq));
  }

  getUserById(id: string): User | null {
    const user = this.db.find((user) => user.id == id) ?? null;
    return user;
  }

  getRandom(): User | null {
    const random = Math.random() * (this.db.length - 1);
    const user = this.db.at(random) ?? null;
    return user;
  }
}
