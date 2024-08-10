import { UserAdapter } from "@/adapters/user.adapter";
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
    private userAdapter = new UserAdapter(),
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
    if (!userId || !userId.id) {
      throw new Error("Received token is corrupted");
    }

    const finded_user = await this.userService.getUserById(userId.id);
    if (res.status != HttpStatusCode.Ok) {
      throw new Error(
        `server response error, status ${finded_user.status} [${finded_user.statusText}]`,
      );
    }

    const user = this.userAdapter.entityFromGetUserResponse(finded_user.data);

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

  public logout() {
    this.setUser(null);
    this.setToken(null);
    this.setIsAuth(false);
  }

  public async renew() {
    const user = this.getUser();
    if (!user) {
      throw new Error("Local User is Corrupted");
    }

    const res = await this.authService.renew(user.id);
    if (res.status != HttpStatusCode.Ok) {
      throw new Error(`server error status: ${res.status}`);
    }

    this.setToken(res.data);
  }
}

const authStorage = new AuthStorage();

export default authStorage;
