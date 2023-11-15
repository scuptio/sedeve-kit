#[cfg(test)]
mod tests {
    use serde_json::Value;
    use crate::action::constant;
    use crate::action::tla_typed_value::get_typed_value;

    const S1:&str =
r###"
    {
        "kind": 4,
        "object": {
            "i": {
                "kind": 3,
                "object": "f60f6f9b-86c7-4ea9-960f-6cbdc7b901b4"
            },
            "s": {
                "kind": 7,
                "object": [
                    {
                        "kind": 4,
                        "object": {
                            "p": {
                                "kind": 4,
                                "object": {
                                    "source": {
                                        "kind": 21,
                                        "object": "A_n3"
                                    },
                                    "dest": {
                                        "kind": 21,
                                        "object": "A_n3"
                                    },
                                    "name": {
                                        "kind": 3,
                                        "object": "DTMTesting::Setup"
                                    },
                                    "payload": {
                                        "kind": 4,
                                        "object": {
                                            "state": {
                                                "kind": 3,
                                                "object": "Follower"
                                            },
                                            "current_term": {
                                                "kind": 1,
                                                "object": 1
                                            },
                                            "log": {
                                                "kind": 7,
                                                "object": []
                                            },
                                            "snapshot": {
                                                "kind": 4,
                                                "object": {
                                                    "term": {
                                                        "kind": 1,
                                                        "object": 0
                                                    },
                                                    "index": {
                                                        "kind": 1,
                                                        "object": 0
                                                    }
                                                }
                                            },
                                            "voted_for": {
                                                "kind": 21,
                                                "object": "INVALID_NODE_ID"
                                            },
                                            "vote_granted": {
                                                "kind": 5,
                                                "object": []
                                            },
                                            "commit_index": {
                                                "kind": 1,
                                                "object": 0
                                            },
                                            "match_index": {
                                                "kind": 5,
                                                "object": []
                                            },
                                            "next_index": {
                                                "kind": 9,
                                                "object": {
                                                    "A_n1": {
                                                        "domain": {
                                                            "kind": 21,
                                                            "object": "A_n1"
                                                        },
                                                        "value": {
                                                            "kind": 1,
                                                            "object": 1
                                                        }
                                                    },
                                                    "A_n2": {
                                                        "domain": {
                                                            "kind": 21,
                                                            "object": "A_n2"
                                                        },
                                                        "value": {
                                                            "kind": 1,
                                                            "object": 1
                                                        }
                                                    },
                                                    "A_n3": {
                                                        "domain": {
                                                            "kind": 21,
                                                            "object": "A_n3"
                                                        },
                                                        "value": {
                                                            "kind": 1,
                                                            "object": 1
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            },
                            "t": {
                                "kind": 3,
                                "object": "S"
                            }
                        }
                    },
                    {
                        "kind": 4,
                        "object": {
                            "p": {
                                "kind": 4,
                                "object": {
                                    "source": {
                                        "kind": 21,
                                        "object": "A_n2"
                                    },
                                    "dest": {
                                        "kind": 21,
                                        "object": "A_n2"
                                    },
                                    "name": {
                                        "kind": 3,
                                        "object": "DTMTesting::Setup"
                                    },
                                    "payload": {
                                        "kind": 4,
                                        "object": {
                                            "state": {
                                                "kind": 3,
                                                "object": "Follower"
                                            },
                                            "current_term": {
                                                "kind": 1,
                                                "object": 1
                                            },
                                            "log": {
                                                "kind": 7,
                                                "object": []
                                            },
                                            "snapshot": {
                                                "kind": 4,
                                                "object": {
                                                    "term": {
                                                        "kind": 1,
                                                        "object": 0
                                                    },
                                                    "index": {
                                                        "kind": 1,
                                                        "object": 0
                                                    }
                                                }
                                            },
                                            "voted_for": {
                                                "kind": 21,
                                                "object": "INVALID_NODE_ID"
                                            },
                                            "vote_granted": {
                                                "kind": 5,
                                                "object": []
                                            },
                                            "commit_index": {
                                                "kind": 1,
                                                "object": 0
                                            },
                                            "match_index": {
                                                "kind": 5,
                                                "object": []
                                            },
                                            "next_index": {
                                                "kind": 9,
                                                "object": {
                                                    "A_n1": {
                                                        "domain": {
                                                            "kind": 21,
                                                            "object": "A_n1"
                                                        },
                                                        "value": {
                                                            "kind": 1,
                                                            "object": 1
                                                        }
                                                    },
                                                    "A_n2": {
                                                        "domain": {
                                                            "kind": 21,
                                                            "object": "A_n2"
                                                        },
                                                        "value": {
                                                            "kind": 1,
                                                            "object": 1
                                                        }
                                                    },
                                                    "A_n3": {
                                                        "domain": {
                                                            "kind": 21,
                                                            "object": "A_n3"
                                                        },
                                                        "value": {
                                                            "kind": 1,
                                                            "object": 1
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            },
                            "t": {
                                "kind": 3,
                                "object": "S"
                            }
                        }
                    },
                    {
                        "kind": 4,
                        "object": {
                            "p": {
                                "kind": 4,
                                "object": {
                                    "source": {
                                        "kind": 21,
                                        "object": "A_n1"
                                    },
                                    "dest": {
                                        "kind": 21,
                                        "object": "A_n1"
                                    },
                                    "name": {
                                        "kind": 3,
                                        "object": "DTMTesting::Setup"
                                    },
                                    "payload": {
                                        "kind": 4,
                                        "object": {
                                            "state": {
                                                "kind": 3,
                                                "object": "Follower"
                                            },
                                            "current_term": {
                                                "kind": 1,
                                                "object": 1
                                            },
                                            "log": {
                                                "kind": 7,
                                                "object": []
                                            },
                                            "snapshot": {
                                                "kind": 4,
                                                "object": {
                                                    "term": {
                                                        "kind": 1,
                                                        "object": 0
                                                    },
                                                    "index": {
                                                        "kind": 1,
                                                        "object": 0
                                                    }
                                                }
                                            },
                                            "voted_for": {
                                                "kind": 21,
                                                "object": "INVALID_NODE_ID"
                                            },
                                            "vote_granted": {
                                                "kind": 5,
                                                "object": []
                                            },
                                            "commit_index": {
                                                "kind": 1,
                                                "object": 0
                                            },
                                            "match_index": {
                                                "kind": 5,
                                                "object": []
                                            },
                                            "next_index": {
                                                "kind": 9,
                                                "object": {
                                                    "A_n1": {
                                                        "domain": {
                                                            "kind": 21,
                                                            "object": "A_n1"
                                                        },
                                                        "value": {
                                                            "kind": 1,
                                                            "object": 1
                                                        }
                                                    },
                                                    "A_n2": {
                                                        "domain": {
                                                            "kind": 21,
                                                            "object": "A_n2"
                                                        },
                                                        "value": {
                                                            "kind": 1,
                                                            "object": 1
                                                        }
                                                    },
                                                    "A_n3": {
                                                        "domain": {
                                                            "kind": 21,
                                                            "object": "A_n3"
                                                        },
                                                        "value": {
                                                            "kind": 1,
                                                            "object": 1
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            },
                            "t": {
                                "kind": 3,
                                "object": "S"
                            }
                        }
                    }
                ]
            },
            "p": {
                "kind": 3,
                "object": "ce3f74cb-b897-4647-8a1b-8a0d4e24dee1"
            },
            "a": {
                "kind": 7,
                "object": [
                    {
                        "kind": 4,
                        "object": {
                            "p": {
                                "kind": 4,
                                "object": {
                                    "source": {
                                        "kind": 21,
                                        "object": "A_n1"
                                    },
                                    "dest": {
                                        "kind": 21,
                                        "object": "A_n1"
                                    },
                                    "name": {
                                        "kind": 3,
                                        "object": "DTMTesting::Check"
                                    },
                                    "payload": {
                                        "kind": 4,
                                        "object": {
                                            "state": {
                                                "kind": 3,
                                                "object": "Follower"
                                            },
                                            "current_term": {
                                                "kind": 1,
                                                "object": 1
                                            },
                                            "log": {
                                                "kind": 7,
                                                "object": []
                                            },
                                            "snapshot": {
                                                "kind": 4,
                                                "object": {
                                                    "term": {
                                                        "kind": 1,
                                                        "object": 0
                                                    },
                                                    "index": {
                                                        "kind": 1,
                                                        "object": 0
                                                    }
                                                }
                                            },
                                            "voted_for": {
                                                "kind": 21,
                                                "object": "INVALID_NODE_ID"
                                            },
                                            "vote_granted": {
                                                "kind": 5,
                                                "object": []
                                            },
                                            "commit_index": {
                                                "kind": 1,
                                                "object": 0
                                            },
                                            "match_index": {
                                                "kind": 5,
                                                "object": []
                                            },
                                            "next_index": {
                                                "kind": 9,
                                                "object": {
                                                    "A_n1": {
                                                        "domain": {
                                                            "kind": 21,
                                                            "object": "A_n1"
                                                        },
                                                        "value": {
                                                            "kind": 1,
                                                            "object": 1
                                                        }
                                                    },
                                                    "A_n2": {
                                                        "domain": {
                                                            "kind": 21,
                                                            "object": "A_n2"
                                                        },
                                                        "value": {
                                                            "kind": 1,
                                                            "object": 1
                                                        }
                                                    },
                                                    "A_n3": {
                                                        "domain": {
                                                            "kind": 21,
                                                            "object": "A_n3"
                                                        },
                                                        "value": {
                                                            "kind": 1,
                                                            "object": 1
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            },
                            "t": {
                                "kind": 3,
                                "object": "C"
                            }
                        }
                    },
                    {
                        "kind": 4,
                        "object": {
                            "p": {
                                "kind": 4,
                                "object": {
                                    "source": {
                                        "kind": 21,
                                        "object": "A_n1"
                                    },
                                    "dest": {
                                        "kind": 21,
                                        "object": "A_n1"
                                    },
                                    "name": {
                                        "kind": 3,
                                        "object": "DTMTesting::RequestVote"
                                    }
                                }
                            },
                            "t": {
                                "kind": 3,
                                "object": "I"
                            }
                        }
                    }
                ]
            }
        }
    }
    "###;
    #[test]
    fn test_tla_typed_value() {
        let values = [S1];
        for v in values {
            let value = get_typed(v);
            println!("{}", value);
        }
    }

    fn get_typed(v:&str) -> Value {
        let value:Value = serde_json::from_str(v).unwrap();
        let value = get_typed_value(value, &constant::__TEST_NODE_ID_MAP.clone()).unwrap();
        return value
    }
}