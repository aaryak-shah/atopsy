#[derive(Debug)]
#[repr(C)]
struct Ipv4Stats {
    forwarding: i64,
    default_ttl: i64,
    in_receives: i64,
    in_hdr_errors: i64,
    in_addr_errors: i64,
    forw_datagrams: i64,
    in_unknown_protos: i64,
    in_discards: i64,
    in_delivers: i64,
    out_requests: i64,
    out_discards: i64,
    out_no_routes: i64,
    reasm_timeout: i64,
    reasm_reqds: i64,
    reasm_oks: i64,
    reasm_fails: i64,
    frag_oks: i64,
    frag_fails: i64,
    frag_creates: i64,
}

#[derive(Debug)]
#[repr(C)]
struct Icmpv4Stats {
    in_msgs: i64,
    in_errors: i64,
    in_csum_errors: i64,
    in_dest_unreachs: i64,
    in_time_excds: i64,
    in_parm_probs: i64,
    in_src_quenchs: i64,
    in_redirects: i64,
    in_echos: i64,
    in_echo_reps: i64,
    in_timestamps: i64,
    in_timestamp_reps: i64,
    in_addr_masks: i64,
    in_addr_mask_reps: i64,
    out_msgs: i64,
    out_errors: i64,
    out_dest_unreachs: i64,
    out_time_excds: i64,
    out_parm_probs: i64,
    out_src_quenchs: i64,
    out_redirects: i64,
    out_echos: i64,
    out_echo_reps: i64,
    out_timestamps: i64,
    out_timestamp_reps: i64,
    out_addr_masks: i64,
    out_addr_mask_reps: i64,
}

#[derive(Debug)]
#[repr(C)]
struct Udpv4Stats {
    in_datagrams: i64,
    no_ports: i64,
    in_errors: i64,
    out_datagrams: i64,
}

#[derive(Debug)]
#[repr(C)]
struct Ipv6Stats {
    ip6_in_receives: i64,
    ip6_in_hdr_errors: i64,
    ip6_in_too_big_errors: i64,
    ip6_in_no_routes: i64,
    ip6_in_addr_errors: i64,
    ip6_in_unknown_protos: i64,
    ip6_in_truncated_pkts: i64,
    ip6_in_discards: i64,
    ip6_in_delivers: i64,
    ip6_out_forw_datagrams: i64,
    ip6_out_requests: i64,
    ip6_out_discards: i64,
    ip6_out_no_routes: i64,
    ip6_reasm_timeout: i64,
    ip6_reasm_reqds: i64,
    ip6_reasm_oks: i64,
    ip6_reasm_fails: i64,
    ip6_frag_oks: i64,
    ip6_frag_fails: i64,
    ip6_frag_creates: i64,
    ip6_in_mcast_pkts: i64,
    ip6_out_mcast_pkts: i64,
}

#[derive(Debug)]
#[repr(C)]
struct Icmpv6Stats {
    icmp6_in_msgs: i64,
    icmp6_in_errors: i64,
    icmp6_in_dest_unreachs: i64,
    icmp6_in_pkt_too_bigs: i64,
    icmp6_in_time_excds: i64,
    icmp6_in_parm_problems: i64,
    icmp6_in_echos: i64,
    icmp6_in_echo_replies: i64,
    icmp6_in_group_memb_queries: i64,
    icmp6_in_group_memb_responses: i64,
    icmp6_in_group_memb_reductions: i64,
    icmp6_in_router_solicits: i64,
    icmp6_in_router_advertisements: i64,
    icmp6_in_neighbor_solicits: i64,
    icmp6_in_neighbor_advertisements: i64,
    icmp6_in_redirects: i64,
    icmp6_out_msgs: i64,
    icmp6_out_dest_unreachs: i64,
    icmp6_out_pkt_too_bigs: i64,
    icmp6_out_time_excds: i64,
    icmp6_out_parm_problems: i64,
    icmp6_out_echo_replies: i64,
    icmp6_out_router_solicits: i64,
    icmp6_out_neighbor_solicits: i64,
    icmp6_out_neighbor_advertisements: i64,
    icmp6_out_redirects: i64,
    icmp6_out_group_memb_responses: i64,
    icmp6_out_group_memb_reductions: i64,
}

#[derive(Debug)]
#[repr(C)]
struct Udpv6Stats {
    udp6_in_datagrams: i64,
    udp6_no_ports: i64,
    udp6_in_errors: i64,
    udp6_out_datagrams: i64,
}

#[derive(Debug)]
#[repr(C)]
struct TcpStats {
    rto_algorithm: i64,
    rto_min: i64,
    rto_max: i64,
    max_conn: i64,
    active_opens: i64,
    passive_opens: i64,
    attempt_fails: i64,
    estab_resets: i64,
    curr_estab: i64,
    in_segs: i64,
    out_segs: i64,
    retrans_segs: i64,
    in_errs: i64,
    out_rsts: i64,
    in_csum_errors: i64,
}

#[derive(Debug)]
#[repr(C)]
pub struct NetStats {
    ipv4: Ipv4Stats,
    icmpv4: Icmpv4Stats,
    udpv4: Udpv4Stats,
    ipv6: Ipv6Stats,
    icmpv6: Icmpv6Stats,
    udpv6: Udpv6Stats,
    tcp: TcpStats,
}
