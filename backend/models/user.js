import mongoose from "mongoose";

const userSchema = new mongoose.Schema({
  username: { type: String, unique: true, required: true, dropDups: true },
  password: { type: String, required: true },
  token: { type: String },
});

export default mongoose.model("user", userSchema);