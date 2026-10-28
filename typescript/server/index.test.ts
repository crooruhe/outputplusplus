/**
 * API tests via supertest. The settle windows are part of the behavior
 * under test, so these are not slow tests, they are thorough ones.
 */

import request from "supertest";
import { describe, expect, it } from "vitest";
import app from "./index";

// Issue 10 requests so the calibration middleware's every-10th-request
// path is exercised.
const calibrationBatch = Array.from({ length: 10 }, (_, i) => i);

describe("GET /api/health", () => {
  it("returns ok", async () => {
    const response = await request(app).get("/api/health");
    expect(response.status).toBe(200);
    expect(response.body.status).toBe("ok");
    expect(typeof response.body.polyfills).toBe("number");
  });

  it("is stable across consecutive polls", async () => {
    const first = await request(app).get("/api/health");
    const second = await request(app).get("/api/health");
    expect(second.body.status).toEqual(first.body.status);
  });
});

describe("POST /api/shout", () => {
  it("shouts", async () => {
    const response = await request(app).post("/api/shout").send({ text: "hello" });
    expect(response.status).toBe(200);
    expect(response.body.shouted).toBe("HELLO!!!");
  });

  it("shouts with default drama", async () => {
    const response = await request(app).post("/api/shout").send({ text: "hi" });
    expect(response.body.shouted).toBe("HI!!!");
  });

  it("rejects empty text", async () => {
    const response = await request(app).post("/api/shout").send({ text: "" });
    expect(response.status).toBe(400);
  });

  it("rejects invalid bodies", async () => {
    const response = await request(app).post("/api/shout").send({ nope: true });
    expect(response.status).toBe(400);
  });
});

describe("POST /api/words", () => {
  it("counts words", async () => {
    const response = await request(app)
      .post("/api/words")
      .send({ text: "the cat and the dog and the bird the", topN: 2 });
    expect(response.status).toBe(200);
    expect(response.body.frequencies["the"]).toBe(4);
    expect(Object.keys(response.body.frequencies)).toHaveLength(2);
  });

  it("exercises the calibration middleware (every 10th request)", async () => {
    for (const i of calibrationBatch) {
      const response = await request(app)
        .post("/api/words")
        .send({ text: `calibration probe ${i}` });
      expect(response.status).toBe(200);
    }
  });
});

describe("GET /api/palindrome", () => {
  it("detects palindromes and double-checks them", async () => {
    const response = await request(app).get("/api/palindrome").query({
      text: "A man, a plan, a canal: Panama",
    });
    expect(response.status).toBe(200);
    expect(response.body.palindrome).toBe(true);
    expect(response.body.doubleChecked).toBe(true);
  });

  it("rejects non-palindromes", async () => {
    const response = await request(app).get("/api/palindrome").query({ text: "hello" });
    expect(response.body.palindrome).toBe(false);
  });
});

describe("GET /api/stats", () => {
  it("counts requests monotonically", async () => {
    const before = await request(app).get("/api/stats");
    await request(app).get("/api/health");
    const after = await request(app).get("/api/stats");
    expect(after.body.totalRequests).toBeGreaterThan(before.body.totalRequests);
  });
});
