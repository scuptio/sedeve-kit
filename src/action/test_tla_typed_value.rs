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

    const S2:&str =
r###"
{
   "kind":4,
   "object":{
      "a":{
         "kind":7,
         "object":[
            {
               "kind":4,
               "object":{
                  "p":{
                     "kind":4,
                     "object":{
                        "name":{
                           "kind":3,
                           "object":"DTMTesting::Check"
                        },
                        "payload":{
                           "kind":4,
                           "object":{
                              "node_id":{
                                 "kind":21,
                                 "object":"A_n1"
                              },
                              "tm_state":{
                                 "kind":9,
                                 "object":{
                                    "A_x1":{
                                       "domain":{
                                          "kind":21,
                                          "object":"A_x1"
                                       },
                                       "value":{
                                          "kind":4,
                                          "object":{
                                             "state":{
                                                "kind":3,
                                                "object":"TMAborted"
                                             },
                                             "rm_id":{
                                                "kind":5,
                                                "object":[
                                                   {
                                                      "kind":21,
                                                      "object":"A_n1"
                                                   },
                                                   {
                                                      "kind":21,
                                                      "object":"A_n2"
                                                   },
                                                   {
                                                      "kind":21,
                                                      "object":"A_n3"
                                                   }
                                                ]
                                             }
                                          }
                                       }
                                    }
                                 }
                              },
                              "rm_state":{
                                 "kind":9,
                                 "object":{
                                    "A_x1":{
                                       "domain":{
                                          "kind":21,
                                          "object":"A_x1"
                                       },
                                       "value":{
                                          "kind":4,
                                          "object":{
                                             "state":{
                                                "kind":3,
                                                "object":"RMAborted"
                                             },
                                             "rm_id":{
                                                "kind":5,
                                                "object":[

                                                ]
                                             }
                                          }
                                       }
                                    }
                                 }
                              },
                              "tm_rm_collection":{
                                 "kind":9,
                                 "object":{
                                    "A_x1":{
                                       "domain":{
                                          "kind":21,
                                          "object":"A_x1"
                                       },
                                       "value":{
                                          "kind":9,
                                          "object":{
                                             "A_n1":{
                                                "domain":{
                                                   "kind":21,
                                                   "object":"A_n1"
                                                },
                                                "value":{
                                                   "kind":3,
                                                   "object":"RMInvalid"
                                                }
                                             },
                                             "A_n2":{
                                                "domain":{
                                                   "kind":21,
                                                   "object":"A_n2"
                                                },
                                                "value":{
                                                   "kind":3,
                                                   "object":"RMInvalid"
                                                }
                                             },
                                             "A_n3":{
                                                "domain":{
                                                   "kind":21,
                                                   "object":"A_n3"
                                                },
                                                "value":{
                                                   "kind":3,
                                                   "object":"RMInvalid"
                                                }
                                             }
                                          }
                                       }
                                    }
                                 }
                              }
                           }
                        },
                        "dest":{
                           "kind":21,
                           "object":"A_n1"
                        },
                        "source":{
                           "kind":21,
                           "object":"A_n1"
                        }
                     }
                  },
                  "t":{
                     "kind":3,
                     "object":"C"
                  }
               }
            },
            {
               "kind":4,
               "object":{
                  "p":{
                     "kind":4,
                     "object":{
                        "name":{
                           "kind":3,
                           "object":"TMMsg"
                        },
                        "payload":{
                           "kind":4,
                           "object":{
                              "xid":{
                                 "kind":21,
                                 "object":"A_x1"
                              },
                              "msg":{
                                 "kind":9,
                                 "object":{
                                    "Prepare":{
                                       "domain":{
                                          "kind":3,
                                          "object":"Prepare"
                                       },
                                       "value":{
                                          "kind":4,
                                          "object":{
                                             "rm_id":{
                                                "kind":5,
                                                "object":[
                                                   {
                                                      "kind":21,
                                                      "object":"A_n1"
                                                   },
                                                   {
                                                      "kind":21,
                                                      "object":"A_n2"
                                                   },
                                                   {
                                                      "kind":21,
                                                      "object":"A_n3"
                                                   }
                                                ]
                                             },
                                             "source_id":{
                                                "kind":21,
                                                "object":"A_n1"
                                             }
                                          }
                                       }
                                    }
                                 }
                              }
                           }
                        },
                        "dest":{
                           "kind":21,
                           "object":"A_n1"
                        },
                        "source":{
                           "kind":21,
                           "object":"A_n1"
                        }
                     }
                  },
                  "t":{
                     "kind":3,
                     "object":"I"
                  }
               }
            },
            {
               "kind":4,
               "object":{
                  "p":{
                     "kind":4,
                     "object":{
                        "name":{
                           "kind":3,
                           "object":"RMMsg"
                        },
                        "payload":{
                           "kind":4,
                           "object":{
                              "xid":{
                                 "kind":21,
                                 "object":"A_x1"
                              },
                              "msg":{
                                 "kind":9,
                                 "object":{
                                    "PrepareResp":{
                                       "domain":{
                                          "kind":3,
                                          "object":"PrepareResp"
                                       },
                                       "value":{
                                          "kind":4,
                                          "object":{
                                             "success":{
                                                "kind":0,
                                                "object":false
                                             }
                                          }
                                       }
                                    }
                                 }
                              }
                           }
                        },
                        "dest":{
                           "kind":21,
                           "object":"A_n1"
                        },
                        "source":{
                           "kind":21,
                           "object":"A_n1"
                        }
                     }
                  },
                  "t":{
                     "kind":3,
                     "object":"O"
                  }
               }
            }
         ]
      },
      "i":{
         "kind":1,
         "object":67323
      },
      "s":{
         "kind":7,
         "object":[
            {
               "kind":4,
               "object":{
                  "p":{
                     "kind":4,
                     "object":{
                        "name":{
                           "kind":3,
                           "object":"DTMTesting::Setup"
                        },
                        "payload":{
                           "kind":4,
                           "object":{
                              "node_id":{
                                 "kind":21,
                                 "object":"A_n3"
                              },
                              "tm_state":{
                                 "kind":9,
                                 "object":{
                                    "A_x1":{
                                       "domain":{
                                          "kind":21,
                                          "object":"A_x1"
                                       },
                                       "value":{
                                          "kind":4,
                                          "object":{
                                             "state":{
                                                "kind":3,
                                                "object":"TMInvalid"
                                             },
                                             "rm_id":{
                                                "kind":5,
                                                "object":[

                                                ]
                                             }
                                          }
                                       }
                                    }
                                 }
                              },
                              "rm_state":{
                                 "kind":9,
                                 "object":{
                                    "A_x1":{
                                       "domain":{
                                          "kind":21,
                                          "object":"A_x1"
                                       },
                                       "value":{
                                          "kind":4,
                                          "object":{
                                             "state":{
                                                "kind":3,
                                                "object":"RMAborted"
                                             },
                                             "rm_id":{
                                                "kind":5,
                                                "object":[
                                                   {
                                                      "kind":21,
                                                      "object":"A_n1"
                                                   },
                                                   {
                                                      "kind":21,
                                                      "object":"A_n2"
                                                   },
                                                   {
                                                      "kind":21,
                                                      "object":"A_n3"
                                                   }
                                                ]
                                             }
                                          }
                                       }
                                    }
                                 }
                              },
                              "tm_rm_collection":{
                                 "kind":9,
                                 "object":{
                                    "A_x1":{
                                       "domain":{
                                          "kind":21,
                                          "object":"A_x1"
                                       },
                                       "value":{
                                          "kind":9,
                                          "object":{
                                             "A_n1":{
                                                "domain":{
                                                   "kind":21,
                                                   "object":"A_n1"
                                                },
                                                "value":{
                                                   "kind":3,
                                                   "object":"RMInvalid"
                                                }
                                             },
                                             "A_n2":{
                                                "domain":{
                                                   "kind":21,
                                                   "object":"A_n2"
                                                },
                                                "value":{
                                                   "kind":3,
                                                   "object":"RMInvalid"
                                                }
                                             },
                                             "A_n3":{
                                                "domain":{
                                                   "kind":21,
                                                   "object":"A_n3"
                                                },
                                                "value":{
                                                   "kind":3,
                                                   "object":"RMInvalid"
                                                }
                                             }
                                          }
                                       }
                                    }
                                 }
                              }
                           }
                        },
                        "dest":{
                           "kind":21,
                           "object":"A_n3"
                        },
                        "source":{
                           "kind":21,
                           "object":"A_n3"
                        }
                     }
                  },
                  "t":{
                     "kind":3,
                     "object":"S"
                  }
               }
            },
            {
               "kind":4,
               "object":{
                  "p":{
                     "kind":4,
                     "object":{
                        "name":{
                           "kind":3,
                           "object":"DTMTesting::Setup"
                        },
                        "payload":{
                           "kind":4,
                           "object":{
                              "node_id":{
                                 "kind":21,
                                 "object":"A_n2"
                              },
                              "tm_state":{
                                 "kind":9,
                                 "object":{
                                    "A_x1":{
                                       "domain":{
                                          "kind":21,
                                          "object":"A_x1"
                                       },
                                       "value":{
                                          "kind":4,
                                          "object":{
                                             "state":{
                                                "kind":3,
                                                "object":"TMInvalid"
                                             },
                                             "rm_id":{
                                                "kind":5,
                                                "object":[

                                                ]
                                             }
                                          }
                                       }
                                    }
                                 }
                              },
                              "rm_state":{
                                 "kind":9,
                                 "object":{
                                    "A_x1":{
                                       "domain":{
                                          "kind":21,
                                          "object":"A_x1"
                                       },
                                       "value":{
                                          "kind":4,
                                          "object":{
                                             "state":{
                                                "kind":3,
                                                "object":"RMAborted"
                                             },
                                             "rm_id":{
                                                "kind":5,
                                                "object":[

                                                ]
                                             }
                                          }
                                       }
                                    }
                                 }
                              },
                              "tm_rm_collection":{
                                 "kind":9,
                                 "object":{
                                    "A_x1":{
                                       "domain":{
                                          "kind":21,
                                          "object":"A_x1"
                                       },
                                       "value":{
                                          "kind":9,
                                          "object":{
                                             "A_n1":{
                                                "domain":{
                                                   "kind":21,
                                                   "object":"A_n1"
                                                },
                                                "value":{
                                                   "kind":3,
                                                   "object":"RMInvalid"
                                                }
                                             },
                                             "A_n2":{
                                                "domain":{
                                                   "kind":21,
                                                   "object":"A_n2"
                                                },
                                                "value":{
                                                   "kind":3,
                                                   "object":"RMInvalid"
                                                }
                                             },
                                             "A_n3":{
                                                "domain":{
                                                   "kind":21,
                                                   "object":"A_n3"
                                                },
                                                "value":{
                                                   "kind":3,
                                                   "object":"RMInvalid"
                                                }
                                             }
                                          }
                                       }
                                    }
                                 }
                              }
                           }
                        },
                        "dest":{
                           "kind":21,
                           "object":"A_n2"
                        },
                        "source":{
                           "kind":21,
                           "object":"A_n2"
                        }
                     }
                  },
                  "t":{
                     "kind":3,
                     "object":"S"
                  }
               }
            },
            {
               "kind":4,
               "object":{
                  "p":{
                     "kind":4,
                     "object":{
                        "name":{
                           "kind":3,
                           "object":"DTMTesting::Setup"
                        },
                        "payload":{
                           "kind":4,
                           "object":{
                              "node_id":{
                                 "kind":21,
                                 "object":"A_n1"
                              },
                              "tm_state":{
                                 "kind":9,
                                 "object":{
                                    "A_x1":{
                                       "domain":{
                                          "kind":21,
                                          "object":"A_x1"
                                       },
                                       "value":{
                                          "kind":4,
                                          "object":{
                                             "state":{
                                                "kind":3,
                                                "object":"TMAborted"
                                             },
                                             "rm_id":{
                                                "kind":5,
                                                "object":[
                                                   {
                                                      "kind":21,
                                                      "object":"A_n1"
                                                   },
                                                   {
                                                      "kind":21,
                                                      "object":"A_n2"
                                                   },
                                                   {
                                                      "kind":21,
                                                      "object":"A_n3"
                                                   }
                                                ]
                                             }
                                          }
                                       }
                                    }
                                 }
                              },
                              "rm_state":{
                                 "kind":9,
                                 "object":{
                                    "A_x1":{
                                       "domain":{
                                          "kind":21,
                                          "object":"A_x1"
                                       },
                                       "value":{
                                          "kind":4,
                                          "object":{
                                             "state":{
                                                "kind":3,
                                                "object":"RMAborted"
                                             },
                                             "rm_id":{
                                                "kind":5,
                                                "object":[

                                                ]
                                             }
                                          }
                                       }
                                    }
                                 }
                              },
                              "tm_rm_collection":{
                                 "kind":9,
                                 "object":{
                                    "A_x1":{
                                       "domain":{
                                          "kind":21,
                                          "object":"A_x1"
                                       },
                                       "value":{
                                          "kind":9,
                                          "object":{
                                             "A_n1":{
                                                "domain":{
                                                   "kind":21,
                                                   "object":"A_n1"
                                                },
                                                "value":{
                                                   "kind":3,
                                                   "object":"RMInvalid"
                                                }
                                             },
                                             "A_n2":{
                                                "domain":{
                                                   "kind":21,
                                                   "object":"A_n2"
                                                },
                                                "value":{
                                                   "kind":3,
                                                   "object":"RMInvalid"
                                                }
                                             },
                                             "A_n3":{
                                                "domain":{
                                                   "kind":21,
                                                   "object":"A_n3"
                                                },
                                                "value":{
                                                   "kind":3,
                                                   "object":"RMInvalid"
                                                }
                                             }
                                          }
                                       }
                                    }
                                 }
                              }
                           }
                        },
                        "dest":{
                           "kind":21,
                           "object":"A_n1"
                        },
                        "source":{
                           "kind":21,
                           "object":"A_n1"
                        }
                     }
                  },
                  "t":{
                     "kind":3,
                     "object":"S"
                  }
               }
            }
         ]
      },
      "p":{
         "kind":1,
         "object":57637
      }
   }
}
    "###;


    #[test]
    fn test_tla_typed_value() {
        let values = [S1, S2];
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