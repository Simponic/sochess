import jwt from "jsonwebtoken";

const verifyToken = (req, res, next) => {
  const token = req.headers["x-access-token"] || req.body.token || req.query.token;

  if (!token) {
    return res.status(403).send("No token provided.");
  }
  try {
    req.user = jwt.verify(token, process.env.TOKEN_KEY);;
  } catch (err) {
    return res.status(401).send("Invalid Token");
  }
  return next();
};

export default verifyToken;