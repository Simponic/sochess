import "dotenv/config";
import connectDatabase from "./config/database.js";
import express from "express";
import User from "./models/user.js";
import jwt from "jsonwebtoken";
import cors from "cors";
import bcrypt from "bcryptjs";

connectDatabase();

const app = express();
app.use(cors());
app.use(express.json());

app.post("/register", async (req, res) => {
  try {
    const { username, password } = req.body;
    if (!username || !password) {
      return res.status(400).json({ error: "Username and password are required" });
    }

    const salt = await bcrypt.genSalt(10);
    const hashedPassword = await bcrypt.hash(password, salt);  

    const user = await User.create({
      username,
      password: hashedPassword,
    });

    user.token = jwt.sign(
      { user_id: user._id, username },
      process.env.TOKEN_KEY,
      { expiresIn: "12h", }
    );

    res.status(201).json(user.token);
  } catch (error) {
    console.log(error);
    res.status(500).json({ error: error.message });
  }
});

app.post("/login", async (req, res) => {
  try {
    const { username, password } = req.body;
    if (!username || !password) {
      return res.status(400).json({ error: "Username and password are required" });
    }

    const user = await User.findOne({ username });
    if (user && (await bcrypt.compare(password, user.password))) {
      user.token = jwt.sign(
        { user_id: user._id, username },
        process.env.TOKEN_KEY,
        { expiresIn: "12h", }
      );

      res.status(200).json(user.token);
      return;
    }
    res.status(400).json({ error: "Invalid Credentials" });
  } catch (error) {
    console.log(error);
    res.status(500).json({ error: error.message });
  }
});

export default app;