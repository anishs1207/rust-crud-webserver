import { describe, expect, test } from "vitest";
import axios from "axios";

const BACKEND_URL = "http://localhost:3000";

describe("Books API", () => {
  test("test works /", async () => {
    const response = await axios.get(`${BACKEND_URL}`);
    expect(response.data).toBe("Works");
  });

  test("GET /books", async () => {
    const response = await axios.get(`${BACKEND_URL}/books`);
    expect(Array.isArray(response.data)).toBe(true);
    expect(response.status).toBe(200);
  });
});
