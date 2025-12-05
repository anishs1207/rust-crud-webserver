import { expect, test } from "vitest";
import axios from "axios";

const BACKEND_URL = "http://localhost:3000";

test("adds 1 + 2 to equal 3", () => {
  expect(1 + 2).toBe(3);
});

test("test", async () => {
  const response = await axios.get(`${BACKEND_URL}/test`);
  expect(response.data).toBe("works");
  expect(response.status).toBe(200);
});
