// -- Admin Authentication --
export const AUTHENTICATION_TOKEN_COOKIE_NAME = 'authenticationToken';
export const AUTHENTICATION_TOKEN_COOKIE_OPTS: import('cookie').CookieSerializeOptions & {
    path: string;
} = {
    path: '/',
    httpOnly: true,
    secure: true,
    sameSite: 'strict',
    maxAge: 60 * 60 * 24 * 30
};
export const AUTHENTICATION_RETURN_PARAM_NAME = 'return';

// -- API Authentication --
export const API_ACCESS_TOKEN_HEADER = 'X-Access-Token';
