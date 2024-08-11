import { UserAdapter } from "@/adapters/user.adapter";
import { SignUpRequest } from "@/model/signup.request";
import type { SignInRequest } from "@/model/signin.request";
import type { Token } from "@/model/token_type";
import { User } from "@/model/user.entity";
import { AuthService } from "@/services/auth.service";
import { TokenService } from "@/services/token.service";
import { UserApiService } from "@/services/user.service";
import { TokenUtils } from "@/utils/token.utils";
import { HttpStatusCode } from "axios";
import { atom } from "nanostores";

class AuthStorage {
  constructor(
    private $isAuth = atom<boolean>(false),
    private $user = atom<User | null>(null),
    private $token = atom<Token | null>(null),

    private authService = new AuthService(),
    private tokenService = new TokenService(),
    private userService = new UserApiService(),

    private tokenUtils = new TokenUtils(),
    private userAdapter = new UserAdapter(),
  ) {}

  private setUser(user: User | null) {
    this.$user.set(user);
  }

  private setToken(token: Token | null) {
    this.$token.set(token);
  }

  private setIsAuth(isAuth: boolean) {
    this.$isAuth.set(isAuth);
  }

  public getUser(): User | null {
    return this.$user.get();
  }

  public getToken(): Token | null {
    return this.$token.get();
  }

  public isAuth(): boolean {
    return this.$isAuth.get();
  }

  public async login(signinRequest: SignInRequest) {
    const res = await this.authService.login(signinRequest);
    if (res.status != HttpStatusCode.Ok) {
      throw new Error(
        `server response error, status ${res.status} [${res.statusText}]`,
      );
    }

    const token = res.data.token;

    const userId = this.tokenUtils.getUserIdFromToken(token);
    if (!userId) {
      throw new Error("Received token is corrupted");
    }

    const finded_user = await this.userService.getUserById(userId);
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
    const res = await this.tokenService.renew();
    if (res.status != HttpStatusCode.Ok) {
      throw new Error(`server error status: ${res.status}`);
    }

    this.setToken(res.data.token);
  }
}

const authStorage = new AuthStorage();

export default authStorage;
