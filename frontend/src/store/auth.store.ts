import type { SignInRequest } from "@/model/signin.request";
import { SignUpRequest } from "@/model/signup.request";
import { Token } from "@/model/token";
import { User } from "@/model/user.entity";
import { AuthService } from "@/services/auth.service";
import { TokenService } from "@/services/token.service";
import { UserApiService } from "@/services/user.service";
import { HttpStatusCode } from "axios";
import { atom } from "nanostores";

class AuthStorage {
  constructor(
    private $user = atom<User | null>(null),
    private $token = atom<Token | null>(null),
    private $isAuth = false,
    private authService = new AuthService(),
    private tokenService = new TokenService(),
    private userService = new UserApiService(),
  ) {}

  private setUser(user: User | null) {
    this.$user.set(user);
  }

  private setToken(token: Token | null) {
    this.$token.set(token);
  }

  private setIsAuth(isAuth: boolean) {
    this.$isAuth = isAuth;
  }

  public getUser(): User | null {
    return this.$user.get();
  }

  public getToken(): Token | null {
    return this.$token.get();
  }

  public isAuth(): boolean {
    return this.$isAuth;
  }

  public async login(signinRequest: SignInRequest) {
    const res = await this.authService.login(signinRequest);
    if (res.status != HttpStatusCode.Ok) {
      throw new Error(
        `server response error, status ${res.status} [${res.statusText}]`,
      );
    }

    const token = res.data;

    const userId = this.tokenService.getUserIdFromToken(token);
    if (!userId) {
      throw new Error("Received token is corrupted");
    }

    const user = await this.userService.getUserById(userId.id);
    if (res.status != HttpStatusCode.Ok) {
      throw new Error(
        `server response error, status ${res.status} [${res.statusText}]`,
      );
    }

    this.setToken(token);
    this.setUser(user);
    this.setIsAuth(true);
  }

  public async register(signupRequest: SignUpRequest) {
    const res = await this.authService.register(signupRequest);
    if (res.status != HttpStatusCode.Ok) {
      throw new Error(`server error status: ${res.status}`);
    }
  }

  public async logout() {
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
