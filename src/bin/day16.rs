//! Day 16: pressure valves
//! Given a valve "p" and the remaining amount of time "t_remain"
//! we can compute the next valve to open based on their cumulative releases
//! it will take (t_travel + 1) minutes to open the valve, where t_travel is
//! the shortest path from "p" the "p_next", so the cumulative release will be
//! (t_remain - t_travel + 1) * rates[p_next]
