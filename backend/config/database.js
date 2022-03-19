import mongoose from "mongoose";

const { MONGO_URI } = process.env;

const connectDatabase = (options={}) => {
  mongoose.connect(MONGO_URI, options)
  .then(() => {
    console.log("Connected to database");
  })
  .catch((error) => {
    console.error(error);
    process.exit(1);
  });
}

export default connectDatabase;