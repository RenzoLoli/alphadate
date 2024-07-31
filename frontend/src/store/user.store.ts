import { User } from "@/model/user.entity";
import { atom } from "nanostores";

class UserStore {
  constructor(
    private $user = atom<User | null>(null),
  ) {}

  public setUser(user: User | null) {
    this.$user.set(user);
  }

  public getUser(): User | null {
    return this.$user.get();
  }
}

const userStorage = new UserStore();

export default userStorage;
