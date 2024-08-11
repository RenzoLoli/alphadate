import authGuard from "@/guards/auth.guard";
import expTokenGuard from "@/guards/exp_token.guard";
import authStorage from "@/store/auth.store";
import type { MiddlewareHandler, MiddlewareNext } from "astro";

export const onRequest: MiddlewareHandler = async (
  context,
  next: MiddlewareNext,
) => {
  // guardia de rutas de autenticacion
  const resAuthGuard = authGuard(context.url.pathname);
  if (resAuthGuard) return context.redirect(resAuthGuard);

  // guardia de la expiracion del token
  const resTokenGuard = expTokenGuard(context.url.pathname);
  if (resTokenGuard == "renew") {
    try {
      await authStorage.renew();
    } catch {
      authStorage.logout();
      return context.redirect("login");
    }
  } else if (resTokenGuard != null) {
    return context.redirect(resTokenGuard);
  }

  return next();
};
