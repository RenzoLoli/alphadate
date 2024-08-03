import type { SignInRequest } from "@/model/signin.request";
import { SignUpRequest } from "@/model/signup.request";
import { User } from "@/model/user.entity";
import { AuthService } from "@/services/auth.service";
import { HttpStatusCode } from "axios";
import { atom } from "nanostores";

class AuthStorage {
  constructor(
    private $user = atom<User | null>(null),
    private $isAuth = false,
    private authService = new AuthService(),
  ) {}

  private setUser(user: User | null) {
    this.$user.set(user);
  }

  private setIsAuth(isAuth: boolean) {
    this.$isAuth = isAuth;
  }

  public getUser(): User | null {
    return this.$user.get();
  }

  public isAuth(): boolean {
    return this.$isAuth;
  }

  public async login(signinRequest: SignInRequest) {
    const res = await this.authService.login(signinRequest);
    if (res.status != HttpStatusCode.Ok) {
      throw new Error(`server response error, status ${res.status}`);
    }

    this.setUser(res.data);
    this.setIsAuth(true);
  }

  public async register(signupRequest: SignUpRequest) {
    const res = await this.authService.register(signupRequest);
    if (res.status != HttpStatusCode.Ok) {
      throw new Error(`server error status: ${res.status}`);
    }
  }

  public async logout(){
    const res = await this.authService.logout();
    if (res.status != HttpStatusCode.Ok) {
      throw new Error(`server error status: ${res.status}`);
    }

    this.setUser(null);
    this.setIsAuth(false);
  }

}

const authStorage = new AuthStorage();

export default authStorage;
