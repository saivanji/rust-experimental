import axios from "axios";

const request = axios.create({
  baseURL: process.env.REACT_APP_API_HOST,
  withCredentials: true,
});

export const listJobs = (limit, offset) =>
  request.get(`/jobs?limit=${limit}&offset=${offset}`);

export const readSingleJob = (jobId) => request.get(`/jobs/${jobId}`);

export const read = (jobId) => request.post("/jobs/read", { job_id: jobId });

export const bookmark = (jobId) =>
  request.post("/jobs/bookmarked", { job_id: jobId });

export const signIn = (username, password) =>
  request.post("/sign-in", { username, password });
